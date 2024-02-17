#![allow(clippy::extra_unused_lifetimes)]

use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::schema::user_roles;

#[derive(
    Debug, Serialize, Deserialize, Insertable, Queryable, AsChangeset, Identifiable, Clone,
)]
#[diesel(table_name = user_roles)]
#[diesel(primary_key(user_role_id))]
pub struct UserRole {
    pub user_role_id: Uuid,
    pub created_by: Uuid,
    pub role_id: Uuid,
    pub user_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
