use diesel::sql_types::{Text, Uuid as DieselUUID};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, QueryableByName};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::schema::user_permissions;

#[derive(
    Debug, Serialize, Deserialize, Insertable, Queryable, AsChangeset, Identifiable, Clone,
)]
#[diesel(table_name = user_permissions)]
#[diesel(primary_key(user_permission_id))]
pub struct UserPermission {
    pub user_permission_id: Uuid,
    pub created_by: Uuid,
    pub user_id: Uuid,
    pub permission_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(QueryableByName, Serialize)]
pub struct UserPermissionLookUp {
    #[diesel(sql_type = DieselUUID)]
    pub user_permission_id: Uuid,
    #[diesel(sql_type = Text)]
    pub permission_name: String,
    #[diesel(sql_type = Text)]
    pub guard_name: String,
}

#[derive(Deserialize)]
pub struct PermissionsParam {
    pub ids: Vec<Uuid>,
}
