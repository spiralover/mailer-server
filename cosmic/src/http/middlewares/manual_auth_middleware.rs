use std::future::{ready, Ready};

use actix_web::error::ErrorUnauthorized;
use actix_web::web::Data;
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{http, FromRequest, HttpMessage, HttpRequest};
use uuid::Uuid;

use crate::app_state::AppState;
use crate::helpers::auth::{
    decode_auth_token, fetch_pat_user, get_auth_user, make_unauthorized_message,
};
use crate::models::user::UserCacheData;
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
        let pat_prefix = app.auth_pat_prefix.clone();

        let token = token.unwrap();
        let is_pat = token.starts_with(&pat_prefix.clone());

        let user_lookup = match is_pat {
            false => {
                let decoded = decode_auth_token(token.clone(), pat_prefix, app.app_key.clone());

                let claims = match decoded {
                    Ok(c) => c.claims,
                    Err(_) => {
                        return make_message("Invalid auth token");
                    }
                };

                if !claims.is_usable() {
                    return make_message("auth token has expired on {}");
                }

                get_auth_user(claims.sub)
            }

            // PERSONAL ACCESS TOKEN
            true => fetch_pat_user(token.clone()),
        };

        if user_lookup.is_error_or_empty() {
            return make_message(&user_lookup.unwrap_err().to_string());
        }

        let user = user_lookup.unwrap();
        let user_id = user.user_id;

        req.extensions_mut().insert::<Uuid>(user_id);
        req.extensions_mut().insert::<UserCacheData>(user);

        ready(Ok(ManualAuthMiddleware { user_id }))
    }
}
