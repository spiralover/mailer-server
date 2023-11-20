use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::schema::applications;

#[derive(
    Debug, Serialize, Deserialize, Insertable, Queryable, AsChangeset, Identifiable, Clone,
)]
#[diesel(table_name = applications)]
#[diesel(primary_key(application_id))]
pub struct Application {
    pub application_id: Uuid,
    pub created_by: Uuid,
    pub name: String,
    pub code: String,
    pub url: String,
    pub logo: String,
    pub webhook: String,
    pub description: String,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

impl Application {
    pub fn transform_response(&mut self) -> Application {
        self.to_owned()
    }
}

pub enum ApplicationStatus {
    Active,
    Inactive,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationCreateForm {
    pub name: String,
    pub url: String,
    pub code: Option<String>,
    pub webhook: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationUpdateForm {
    pub name: String,
    pub url: String,
    pub webhook: String,
    pub description: String,
}
