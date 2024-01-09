use std::env;

use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;

use crate::services::auth_service::TokenClaims;

#[derive(Serialize, Debug)]
pub struct AuthTokenData {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

pub fn generate_token(
    payload: String,
    header: Option<Header>,
    lifetime: Option<i64>,
) -> AuthTokenData {
    let token_lifetime_in_minutes: i64 = lifetime.unwrap_or_else(|| {
        env::var("MAILER_AUTH_TOKEN_LIFETIME")
            .unwrap()
            .parse()
            .unwrap()
    });

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(token_lifetime_in_minutes)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        exp,
        iat,
        sub: payload,
    };

    let token_header = header.unwrap_or_default();

    let token = encode(
        &token_header,
        &claims,
        &EncodingKey::from_secret(env::var("MAILER_APP_KEY").unwrap().as_ref()),
    )
    .unwrap();

    AuthTokenData {
        access_token: token,
        token_type: "bearer".to_string(),
        expires_in: token_lifetime_in_minutes,
    }
}
