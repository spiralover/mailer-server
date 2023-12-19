use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::schema::app_keys;

#[derive(
    Debug, Serialize, Deserialize, Insertable, Queryable, AsChangeset, Identifiable, Clone,
)]
#[diesel(table_name = app_keys)]
#[diesel(primary_key(app_key_id))]
pub struct AppKey {
    pub app_key_id: Uuid,
    pub created_by: Uuid,
    pub application_id: Uuid,
    pub public_key: String,
    pub private_key: String,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

pub struct AppKeyResponse {
    pub public_key: String,
    pub private_key: String,
}

impl AppKey {
    pub fn transform_response(self) -> AppKeyResponse {
        AppKeyResponse {
            public_key: self.public_key,
            private_key: self.private_key,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateForm {
    pub name: String,
    pub url: String,
    pub webhook: Option<String>,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateForm {
    pub name: String,
    pub url: String,
    pub webhook: Option<String>,
    pub description: String,
}
