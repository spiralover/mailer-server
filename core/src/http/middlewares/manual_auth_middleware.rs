use core::fmt;
use std::env;
use std::future::{ready, Ready};

use actix_web::error::ErrorUnauthorized;
use actix_web::web::Data;
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{http, FromRequest, HttpMessage, HttpRequest};
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

impl fmt::Display for ErrorResponse<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

pub struct ManualAuthMiddleware {
    pub user_id: Uuid,
}

impl FromRequest for ManualAuthMiddleware {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
            });

        if token.is_none() {
            return ready(Err(ErrorUnauthorized(make_unauthorized_response(
                "You are not logged in, please provide token",
            ))));
        }

        let claims = match decode::<TokenClaims>(
            &token.unwrap(),
            &DecodingKey::from_secret(env::var("APP_KEY").unwrap().as_ref()),
            &Validation::default(),
        ) {
            Ok(c) => c.claims,
            Err(_) => {
                return ready(Err(ErrorUnauthorized(make_unauthorized_response(
                    "Invalid auth token",
                ))));
            }
        };

        let user_id = UniqueIdentifier::from_string(claims.sub);
        let pool = req.app_data::<Data<AppState>>().unwrap().database();
        let user_lookup = UserRepository.find_by_id(pool, user_id.uuid());

        if user_lookup.is_error_or_empty() {
            return ready(Err(ErrorUnauthorized(make_unauthorized_response(
                "Invalid auth token, user not found",
            ))));
        }

        req.extensions_mut().insert::<Uuid>(user_id.uuid());

        req.extensions_mut().insert::<User>(user_lookup.unwrap());

        ready(Ok(ManualAuthMiddleware {
            user_id: user_id.uuid(),
        }))
    }
}

fn make_unauthorized_response(message: &str) -> ErrorResponse {
    ErrorResponse {
        success: false,
        status: 401,
        message,
    }
}
