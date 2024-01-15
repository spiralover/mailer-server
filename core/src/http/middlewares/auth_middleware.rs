use std::future::{ready, Ready};

use actix_web::web::Data;
use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    http, Error, HttpMessage, HttpRequest, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use log::{debug, error};
use uuid::Uuid;

use crate::app_state::AppState;
use crate::enums::auth_role::AuthRole;
use crate::helpers::auth::{
    decode_auth_token, fetch_pat_user, get_auth_user, make_unauthorized_message,
};
use crate::models::user::UserCacheData;
use crate::repositories::user_role_repository::UserRoleRepository;
use crate::results::http_result::ErroneousOptionResponse;

#[derive(Clone)]
pub struct AuthMiddleware {
    pub roles: Vec<AuthRole>,
}

impl AuthMiddleware {
    pub fn new(roles: Vec<AuthRole>) -> AuthMiddleware {
        Self { roles }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AuthMiddlewareInternal<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareInternal {
            service,
            roles: self.roles.clone(),
        }))
    }
}

pub struct AuthMiddlewareInternal<S> {
    service: S,
    roles: Vec<AuthRole>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareInternal<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let token = request
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                request
                    .headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
            });

        if token.is_none() {
            //You are not logged in, please provide token
            let response = HttpResponse::Unauthorized()
                .json(make_unauthorized_message(
                    "you are not logged in, please provide token",
                ))
                .map_into_right_body();

            let (req, _pl) = request.into_parts();
            return Box::pin(async { Ok(ServiceResponse::new(req, response)) });
        }

        let app = request.app_data::<Data<AppState>>().unwrap();
        let mut cache = app.services.cache.clone();
        let pat_prefix = app.auth_pat_prefix.clone();

        let token = token.unwrap();
        let is_pat = token.starts_with(&pat_prefix.clone());

        let error_messenger = |req: HttpRequest, msg: &str| {
            let response = HttpResponse::Unauthorized()
                .json(make_unauthorized_message(msg))
                .map_into_right_body();

            Box::pin(async { Ok(ServiceResponse::new(req, response)) })
        };

        let user_lookup = match is_pat {
            // AUTHENTICATION TOKEN
            false => {
                let decoded = decode_auth_token(token.clone(), pat_prefix, app.app_key.clone());

                if let Err(err) = decoded {
                    let (req, _pl) = request.into_parts();
                    debug!("invalid token({}): {:?}", token, err);
                    return error_messenger(req, "invalid authentication token");
                }

                let claims = decoded.unwrap().claims;

                if !claims.is_usable() {
                    let (req, _pl) = request.into_parts();
                    let message = Box::leak(Box::new(format!(
                        "auth token has expired on {}",
                        claims.exp
                    )));
                    return error_messenger(req, message);
                }

                get_auth_user(app.database(), &mut cache, claims.sub)
            }

            // PERSONAL ACCESS TOKEN
            true => fetch_pat_user(app, &mut cache, token.clone()),
        };

        if user_lookup.is_error_or_empty() {
            let (req, _pl) = request.into_parts();
            return error_messenger(req, &user_lookup.unwrap_err().to_string());
        }

        let user = user_lookup.unwrap();

        if !self.roles.is_empty() {
            let roles_result =
                UserRoleRepository.list_role_names_by_user_id(app.database(), user.user_id);
            if roles_result.is_error_or_empty() {
                error!(
                    "failed to fetch user roles: {:?}",
                    roles_result.unwrap_err()
                );

                let (req, _pl) = request.into_parts();
                return error_messenger(req, "Something went wrong trying to authenticate you");
            }

            let mut has_access = false;
            let roles = roles_result.unwrap();
            for role in &self.roles {
                if roles.contains(&role.to_string()) {
                    has_access = true;
                    break;
                }
            }

            if !has_access {
                let (req, _pl) = request.into_parts();
                return error_messenger(
                    req,
                    "You are not authorised to access requested resource(s)",
                );
            }
        }

        request.extensions_mut().insert::<Uuid>(user.user_id);
        request.extensions_mut().insert::<UserCacheData>(user);

        let res = self.service.call(request);

        Box::pin(async move {
            // forwarded responses map to "left" body
            res.await.map(ServiceResponse::map_into_left_body)
        })
    }
}
