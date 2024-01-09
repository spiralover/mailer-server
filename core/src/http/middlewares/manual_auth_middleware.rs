use std::future::{ready, Ready};

use actix_web::error::ErrorUnauthorized;
use actix_web::web::Data;
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{http, FromRequest, HttpMessage, HttpRequest};
use log::error;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::helpers::auth::{decode_auth_token, get_auth_user, make_unauthorized_message};
use crate::models::user::UserCacheData;
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

        let make_message =
            |msg: &str| ready(Err(ErrorUnauthorized(make_unauthorized_message(msg))));

        if token.is_none() {
            return make_message("You are not logged in, please provide token");
        }

        let app = req.app_data::<Data<AppState>>().unwrap();
        let mut redis_service = app.services.redis.clone();
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
                        return make_message("Invalid auth token");
                    }
                };

                if !claims.is_usable() {
                    return make_message(Box::leak(Box::new(format!(
                        "auth token has expired on {}",
                        claims.exp
                    ))));
                }

                get_auth_user(app.database(), redis_service, claims)
            }

            // PERSONAL ACCESS TOKEN
            true => match redis_service.get(raw_token.clone()) {
                Ok(data) => Ok(serde_json::from_str::<UserCacheData>(&data).unwrap()),
                Err(_error) => {
                    let pat_result = PersonaAccessTokenRepository
                        .find_by_token(app.database(), raw_token.clone());

                    match pat_result {
                        Ok(pat) => {
                            if !pat.is_usable() {
                                let msg = Box::leak(Box::new(format!(
                                    "personal access token has expired on {:?}",
                                    pat.expired_at
                                )));

                                let _ = redis_service.delete(raw_token);

                                return make_message(msg);
                            }

                            UserRepository
                                .find_by_id(app.database(), pat.user_id)
                                .map(|user| {
                                    let user = user.into_cache_data();
                                    let _ = redis_service.set(raw_token, user.clone());
                                    user
                                })
                        }
                        Err(err) => {
                            error!("pat error: {:?}", err);
                            return make_message("failed to authenticate personal access token");
                        }
                    }
                }
            },
        };

        if user_lookup.is_error_or_empty() {
            return make_message("Invalid auth token, user not found");
        }

        let user = user_lookup.unwrap();
        let user_id = user.user_id;

        req.extensions_mut().insert::<Uuid>(user_id);
        req.extensions_mut().insert::<UserCacheData>(user);

        ready(Ok(ManualAuthMiddleware { user_id }))
    }
}
