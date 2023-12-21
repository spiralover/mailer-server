use std::future::{ready, Ready};

use actix_web::web::Data;
use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    http, Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use log::error;
use serde::Serialize;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::enums::auth_role::AuthRole;
use crate::helpers::auth::decode_auth_token;
use crate::helpers::uuid::UniqueIdentifier;
use crate::models::user::User;
use crate::repositories::personal_access_token_repository::PersonaAccessTokenRepository;
use crate::repositories::user_repository::UserRepository;
use crate::repositories::user_role_repository::UserRoleRepository;
use crate::results::http_result::ErroneousOptionResponse;

#[derive(Debug, Serialize)]
struct ErrorResponse<'a> {
    success: bool,
    status: i32,
    message: &'a str,
}

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
        // Change this to see the change in outcome in the browser.
        // Usually this boolean would be acquired from a password check or other auth verification.

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
                .json(ErrorResponse {
                    success: false,
                    status: 401,
                    message: "you are not logged in, please provide token",
                })
                .map_into_right_body();

            let (req, _pl) = request.into_parts();
            return Box::pin(async { Ok(ServiceResponse::new(req, response)) });
        }

        let app = request.app_data::<Data<AppState>>().unwrap();
        let pat_prefix = app.auth_pat_prefix.clone();

        let raw_token = token.unwrap();
        let is_pat = raw_token.starts_with(&pat_prefix.clone());

        let user_lookup = match is_pat {
            // AUTHENTICATION TOKEN
            false => {
                let decoded = decode_auth_token(raw_token.clone(), pat_prefix, app.app_key.clone());

                if decoded.is_err() {
                    let response = HttpResponse::Unauthorized()
                        .json(ErrorResponse {
                            success: false,
                            status: 401,
                            message: "invalid authentication token",
                        })
                        .map_into_right_body();

                    let (req, _pl) = request.into_parts();
                    return Box::pin(async { Ok(ServiceResponse::new(req, response)) });
                }

                let claims = decoded.unwrap().claims;

                let user_id = UniqueIdentifier::from_string(claims.sub);
                UserRepository.find_by_id(app.database(), user_id.uuid())
            }

            // PERSONAL ACCESS TOKEN
            true => match PersonaAccessTokenRepository.find_by_token(app.database(), raw_token) {
                Ok(pat) => {
                    if !pat.is_usable() {
                        let message = Box::leak(Box::new(format!(
                            "personal access token has expired on {:?}",
                            pat.expired_at
                        )));

                        let response = HttpResponse::Unauthorized()
                            .json(ErrorResponse {
                                success: false,
                                status: 401,
                                message,
                            })
                            .map_into_right_body();

                        let (req, _pl) = request.into_parts();
                        return Box::pin(async { Ok(ServiceResponse::new(req, response)) });
                    }

                    UserRepository.find_by_id(app.database(), pat.user_id)
                }
                Err(err) => {
                    error!("pat error: {:?}", err);
                    let response = HttpResponse::Unauthorized()
                        .json(ErrorResponse {
                            success: false,
                            status: 401,
                            message: "failed to authenticate personal access token",
                        })
                        .map_into_right_body();

                    let (req, _pl) = request.into_parts();
                    return Box::pin(async { Ok(ServiceResponse::new(req, response)) });
                }
            },
        };

        if user_lookup.is_error_or_empty() {
            let response = HttpResponse::Unauthorized()
                .json(ErrorResponse {
                    success: false,
                    status: 401,
                    message: "Invalid auth token, user not found",
                })
                .map_into_right_body();

            let (req, _pl) = request.into_parts();
            return Box::pin(async { Ok(ServiceResponse::new(req, response)) });
        }

        let user = user_lookup.unwrap();

        if !self.roles.is_empty() {
            let roles_result =
                UserRoleRepository.list_role_names_by_user_id(app.database(), user.user_id);
            if roles_result.is_error_or_empty() {
                let response = HttpResponse::Unauthorized()
                    .json(ErrorResponse {
                        success: false,
                        status: 401,
                        message: "Something went wrong trying to authenticate you",
                    })
                    .map_into_right_body();

                error!(
                    "failed to fetch user roles: {:?}",
                    roles_result.unwrap_err()
                );

                let (req, _pl) = request.into_parts();
                return Box::pin(async { Ok(ServiceResponse::new(req, response)) });
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
                let response = HttpResponse::Unauthorized()
                    .json(ErrorResponse {
                        success: false,
                        status: 401,
                        message: "You are not authorised to access requested resource(s)",
                    })
                    .map_into_right_body();

                let (req, _pl) = request.into_parts();
                return Box::pin(async { Ok(ServiceResponse::new(req, response)) });
            }
        }

        request.extensions_mut().insert::<Uuid>(user.user_id);
        request.extensions_mut().insert::<User>(user);

        let res = self.service.call(request);

        Box::pin(async move {
            // forwarded responses map to "left" body
            res.await.map(ServiceResponse::map_into_left_body)
        })
    }
}
