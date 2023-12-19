use std::env;
use std::future::{ready, Ready};

use actix_web::web::Data;
use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    http, Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::models::user::User;
use crate::repositories::user_repository::UserRepository;
use crate::results::http_result::ErroneousOptionResponse;
use crate::services::auth_service::TokenClaims;
use crate::uuid::UniqueIdentifier;

#[derive(Debug, Serialize)]
struct ErrorResponse<'a> {
    success: bool,
    status: i32,
    message: &'a str,
}

#[derive(Clone)]
pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}
pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
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
                    message: "You are not logged in, please provide token",
                })
                .map_into_right_body();

            let (req, _pl) = request.into_parts();
            return Box::pin(async { Ok(ServiceResponse::new(req, response)) });
        }

        let decoded = decode::<TokenClaims>(
            &token.unwrap(),
            &DecodingKey::from_secret(env::var("APP_KEY").unwrap().as_ref()),
            &Validation::default(),
        );

        if decoded.is_err() {
            let response = HttpResponse::Unauthorized()
                .json(ErrorResponse {
                    success: false,
                    status: 401,
                    message: "Invalid authentication token",
                })
                .map_into_right_body();

            let (req, _pl) = request.into_parts();
            return Box::pin(async { Ok(ServiceResponse::new(req, response)) });
        }

        let claims = decoded.unwrap().claims;

        let user_id = UniqueIdentifier::from_string(claims.sub);
        let pool = request.app_data::<Data<AppState>>().unwrap().database();
        let user_lookup = UserRepository.find_by_id(pool, user_id.uuid());

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

        request.extensions_mut().insert::<Uuid>(user_id.uuid());

        request
            .extensions_mut()
            .insert::<User>(user_lookup.unwrap());

        let res = self.service.call(request);

        Box::pin(async move {
            // forwarded responses map to "left" body
            res.await.map(ServiceResponse::map_into_left_body)
        })
    }
}
