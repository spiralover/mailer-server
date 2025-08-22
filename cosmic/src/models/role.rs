#![allow(clippy::extra_unused_lifetimes)]

use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString, VariantNames};
use uuid::Uuid;

use super::super::schema::roles;
use validator::Validate;

#[derive(
    Debug, Serialize, Deserialize, Insertable, Queryable, AsChangeset, Identifiable, Clone,
)]
#[diesel(table_name = roles)]
#[diesel(primary_key(role_id))]
pub struct Role {
    pub role_id: Uuid,
    pub created_by: Uuid,
    pub role_name: String,
    pub guard_name: String,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Clone, Display, Debug, EnumString, VariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum RoleStatus {
    Active,
    Inactive,
}

#[derive(Deserialize)]
pub struct RoleParam {
    pub role_id: Uuid,
}

#[derive(Deserialize, Validate)]
pub struct RoleCreateForm {
    #[validate(length(min = 3, max = 150))]
    pub name: String,

    #[validate(length(min = 3, max = 150))]
    pub guard: String,
}
