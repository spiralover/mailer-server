#![allow(clippy::extra_unused_lifetimes)]

use derive_more::Display;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, EnumVariantNames};
use uuid::Uuid;

use super::super::schema::password_resets;

#[derive(
    Debug, Serialize, Deserialize, Insertable, Queryable, AsChangeset, Identifiable, Clone,
)]
#[diesel(table_name = password_resets)]
#[diesel(primary_key(password_reset_id))]
pub struct PasswordReset {
    pub password_reset_id: Uuid,
    pub user_id: Uuid,
    pub email: String,
    pub token: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Clone, Display, Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum PasswordResetStatus {
    AwaitingVerification,
    Completed,
    TokenExpired,
}

pub struct PasswordResetCreateDto {
    pub email: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}
