#![allow(clippy::extra_unused_lifetimes)]

use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::schema::permissions;

#[derive(
    Debug, Serialize, Deserialize, Insertable, Queryable, AsChangeset, Identifiable, Clone,
)]
#[diesel(table_name = permissions)]
#[diesel(primary_key(permission_id))]
pub struct Permission {
    pub permission_id: Uuid,
    pub created_by: Uuid,
    pub permission_name: String,
    pub guard_name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateForm {
    pub name: String,
    pub guard: String,
}

#[derive(Serialize, Queryable)]
pub struct UserPermissionItem {
    pub permission_id: Uuid,
    pub permission_name: String,
    pub guard_name: String,
}
