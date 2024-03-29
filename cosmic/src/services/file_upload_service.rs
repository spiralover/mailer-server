use std::path::PathBuf;
use std::sync::Arc;

use actix_multipart::form::tempfile::TempFile;
use diesel::SaveChangesDsl;
use log::error;
use nanoid::nanoid;
use reqwest::StatusCode;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::enums::app_message::AppMessage::{ErrorMessage, WarningMessageStr};
use crate::enums::entities::Entities;
use crate::helpers::db::DatabaseConnectionHelper;
use crate::helpers::string::string;
use crate::helpers::time::current_timestamp;
use crate::helpers::DBPool;
use crate::models::file_upload::{FileUpload, FileUploadCreateForm, FileUploadData};
use crate::repositories::file_upload_repository::FileUploadRepository;
use crate::results::app_result::FormatAppResult;
use crate::results::AppResult;

pub struct FileUploadService;

impl FileUploadService {
    pub fn create(
        &mut self,
        pool: &DBPool,
        uploader_id: Uuid,
        form: FileUploadCreateForm,
    ) -> AppResult<FileUpload> {
        FileUploadRepository.create(pool, uploader_id, form)
    }

    pub fn upload(
        &mut self,
        app: Arc<AppState>,
        file: TempFile,
        data: FileUploadData,
    ) -> AppResult<FileUpload> {
        let max_file_size = app.max_image_upload_size;
        match file.size {
            0 => {
                return Err(ErrorMessage(
                    string("invalid file"),
                    StatusCode::BAD_REQUEST,
                ));
            }
            length if length > (max_file_size as usize) => {
                let msg = format!(
                    "The uploaded file is too large. Maximum size is {} bytes.",
                    max_file_size
                );

                let msg: &'static str = Box::leak(Box::new(msg));
                return Err(WarningMessageStr(msg));
            }
            _ => {}
        };

        let temp_path = file.file.path();
        let file_name: &str = file
            .file_name
            .as_ref()
            .map(|m| m.as_ref())
            .unwrap_or("null");

        let split_name: Vec<&str> = file_name.split('.').collect();
        if split_name.len() == 1 {
            return Err(WarningMessageStr(
                "Invalid file type, file must have valid extension",
            ));
        }

        let file_ext = *split_name.last().unwrap();
        let new_file_name = format!("{}.{}", nanoid!(), file_ext);

        let mut file_path = PathBuf::from("resources/static/uploads");
        file_path.push(new_file_name.clone());

        let uploaded_file_path = match std::fs::copy(temp_path, file_path.clone()) {
            Ok(_) => {
                std::fs::remove_file(temp_path)?;
                Ok(file_path)
            }
            Err(err) => {
                error!("File Upload Error: {}", err);
                Err(err)
            }
        }?;

        FileUploadService.create(
            app.database(),
            data.uploader_id,
            FileUploadCreateForm {
                owner_id: data.owner_id,
                owner_type: data.owner_type,
                orig_name: file_name.to_string(),
                file_name: new_file_name,
                file_path: uploaded_file_path.to_str().unwrap().to_string(),
                file_ext: file_ext.to_string(),
                description: data.description,
                additional_info: data.additional_info,
                is_temp: data.is_temp,
            },
        )
    }

    #[allow(dead_code)]
    pub fn delete(&mut self, pool: &DBPool, id: Uuid) -> AppResult<FileUpload> {
        FileUploadRepository.delete(pool, id)
    }

    #[allow(dead_code)]
    pub fn mark_temp_as_used(
        &mut self,
        pool: &DBPool,
        mut file: FileUpload,
        owner_type: Entities,
        owner_id: Uuid,
        desc: String,
    ) -> AppResult<FileUpload> {
        file.is_temp = false;
        file.description = Some(desc);
        file.owner_id = owner_id;
        file.owner_type = owner_type.to_string();
        file.updated_at = current_timestamp();
        file.save_changes::<FileUpload>(&mut pool.conn())
            .into_app_result()
    }
}
