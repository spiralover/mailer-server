use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::Entities;

use super::super::schema::file_uploads;

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Insertable,
    Queryable,
    Selectable,
    AsChangeset,
    Identifiable,
    Clone,
)]
#[diesel(table_name = file_uploads)]
#[diesel(primary_key(file_upload_id))]
pub struct FileUpload {
    pub file_upload_id: Uuid,
    pub uploader_id: Uuid,
    pub owner_id: Uuid,
    pub owner_type: String,
    pub orig_name: String,
    pub file_name: String,
    pub file_path: String,
    pub file_ext: String,
    pub description: Option<String>,
    pub additional_info: Option<String>,
    pub is_temp: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

pub struct FileUploadCreateForm {
    pub owner_id: Uuid,
    pub owner_type: Entities,
    pub orig_name: String,
    pub file_name: String,
    pub file_path: String,
    pub file_ext: String,
    pub description: Option<String>,
    pub is_temp: bool,
    pub additional_info: Option<String>,
}

pub struct FileUploadData {
    pub uploader_id: Uuid,
    pub owner_id: Uuid,
    pub owner_type: Entities,
    pub is_temp: bool,
    pub description: Option<String>,
    pub additional_info: Option<String>,
}
