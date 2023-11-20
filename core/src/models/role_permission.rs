use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::schema::role_permissions;

#[derive(
    Debug, Serialize, Deserialize, Insertable, Queryable, AsChangeset, Identifiable, Clone,
)]
#[diesel(table_name = role_permissions)]
#[diesel(primary_key(role_permission_id))]
pub struct RolePermission {
    pub role_permission_id: Uuid,
    pub created_by: Uuid,
    pub role_id: Uuid,
    pub permission_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateForm {
    pub role_id: Uuid,
    pub permission_id: Uuid,
}
