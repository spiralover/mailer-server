use std::future::{ready, Ready};

use actix_web::error::ErrorUnauthorized;
use actix_web::web::Data;
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{http, FromRequest, HttpMessage, HttpRequest};
use log::error;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::enums::app_message::AppMessage;
use crate::helpers::auth::{decode_auth_token, make_unauthorized_message};
use crate::helpers::uuid::UniqueIdentifier;
use crate::models::user::User;
use crate::repositories::personal_access_token_repository::PersonaAccessTokenRepository;
use crate::repositories::user_repository::UserRepository;
use crate::results::http_result::ErroneousOptionResponse;

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
            return ready(Err(ErrorUnauthorized(make_unauthorized_message(
                "You are not logged in, please provide token",
            ))));
        }

        let app = req.app_data::<Data<AppState>>().unwrap();
        let pat_prefix = app.auth_pat_prefix.clone();

        let raw_token = token.unwrap();
        let is_pat = raw_token.starts_with(&pat_prefix.clone());

        let app = req.app_data::<Data<AppState>>().unwrap();

        let user_lookup = match is_pat {
            false => {
                let decoded = decode_auth_token(raw_token.clone(), pat_prefix, app.app_key.clone());

                let claims = match decoded {
                    Ok(c) => c.claims,
                    Err(_) => {
                        return ready(Err(ErrorUnauthorized(make_unauthorized_message(
                            "Invalid auth token",
                        ))));
                    }
                };

                let user_id = UniqueIdentifier::from_string(claims.sub);
                UserRepository.find_by_id(app.database(), user_id.uuid())
            }
            true => match PersonaAccessTokenRepository.find_by_token(app.database(), raw_token) {
                Ok(pat) => {
                    if !pat.is_usable() {
                        let message = Box::leak(Box::new(format!(
                            "personal access token has expired on {:?}",
                            pat.expired_at
                        )));
                        return ready(Err(ErrorUnauthorized(make_unauthorized_message(message))));
                    }

                    UserRepository.find_by_id(app.database(), pat.user_id)
                }
                Err(error) => {
                    match error {
                        AppMessage::DatabaseEntityNotFound => {}
                        _ => error!("pat error: {:?}", error),
                    }

                    return ready(Err(ErrorUnauthorized(make_unauthorized_message(
                        "failed to authenticate personal access token",
                    ))));
                }
            },
        };

        if user_lookup.is_error_or_empty() {
            return ready(Err(ErrorUnauthorized(make_unauthorized_message(
                "Invalid auth token, user not found",
            ))));
        }

        let user = user_lookup.unwrap();
        let user_id = user.user_id;

        req.extensions_mut().insert::<Uuid>(user_id);
        req.extensions_mut().insert::<User>(user);

        ready(Ok(ManualAuthMiddleware { user_id }))
    }
}
