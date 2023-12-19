use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

use super::super::schema::auth_attempts;

#[derive(
    Debug, Serialize, Deserialize, Insertable, Queryable, AsChangeset, Identifiable, Clone,
)]
#[diesel(table_name = auth_attempts)]
#[diesel(primary_key(auth_attempt_id))]
pub struct AuthAttempt {
    pub auth_attempt_id: Uuid,
    pub user_id: Option<Uuid>,
    pub email: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub auth_error: Option<String>,
    pub verification_code: Option<String>,
    pub verification_code_trials: i16,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

pub struct CreateDto {
    pub email: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub user_id: Option<Uuid>,
    pub auth_error: Option<String>,
    pub verification_code: Option<String>,
}

pub enum AuthAttemptStatus {
    LoggedIn,
    LoginDenied,
    InvalidCredential,
    InvalidatedToken,
    PendingVerification,
}

impl Display for AuthAttemptStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let status = match self {
            AuthAttemptStatus::LoggedIn => "logged_in",
            AuthAttemptStatus::LoginDenied => "logged_denied",
            AuthAttemptStatus::InvalidCredential => "invalid_credential",
            AuthAttemptStatus::InvalidatedToken => "invalidated_token",
            AuthAttemptStatus::PendingVerification => "pending_verification",
        };

        write!(f, "{}", status)
    }
}

#[derive(Deserialize)]
pub struct LoginToken {
    pub code: String,
}
