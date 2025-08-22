use std::future::{ready, Ready};
use std::rc::Rc;

use actix_web::web::{block, Data};
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
use crate::results::app_result::ActixBlockResult;
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
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
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
            service: Rc::new(service),
            roles: self.roles.clone(),
        }))
    }
}

pub struct AuthMiddlewareInternal<S> {
    service: Rc<S>,
    roles: Vec<AuthRole>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareInternal<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let svc = Rc::clone(&self.service);
        let roles = self.roles.clone();
        Box::pin(async move {
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
                return Ok(ServiceResponse::new(req, response));
            }

            let app = request.app_data::<Data<AppState>>().unwrap();
            let pat_prefix = app.auth_pat_prefix.clone();

            let token = token.unwrap();
            let is_pat = token.starts_with(&pat_prefix.clone());

            let error_messenger = |req: HttpRequest, msg: &str| {
                let response = HttpResponse::Unauthorized()
                    .json(make_unauthorized_message(msg))
                    .map_into_right_body();

                Ok(ServiceResponse::new(req, response))
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

                    let user_id = claims.sub.clone();
                    block(move || get_auth_user(user_id))
                        .await
                        .into_app_result()
                }

                // PERSONAL ACCESS TOKEN
                true => block(move || fetch_pat_user(token.clone()))
                    .await
                    .into_app_result(),
            };

            if user_lookup.is_error_or_empty() {
                let (req, _pl) = request.into_parts();
                let err = user_lookup.unwrap_err().to_string();
                error!("user lookup error: {:?}", err);
                return error_messenger(req, &err);
            }

            let user: UserCacheData = user_lookup.unwrap();

            if !roles.is_empty() {
                let mut has_access = false;
                let roles = user.roles.clone();
                for role in &roles {
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

            debug!("calling http controller -> method...");

            let res = svc.call(request);
            // forwarded responses map to "left" body
            res.await.map(ServiceResponse::map_into_left_body)
        })
    }
}
