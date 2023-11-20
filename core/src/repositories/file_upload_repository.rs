use std::ops::DerefMut;

use diesel::{ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl, SaveChangesDsl};
use uuid::Uuid;

use crate::helpers::db::{current_timestamp, OptionalResult};
use crate::helpers::db_pagination::{Paginate, PaginationResult};
use crate::helpers::get_db_conn;
use crate::helpers::http::QueryParams;
use crate::models::file_upload::{FileUpload, FileUploadCreateForm};
use crate::models::DBPool;
use crate::results::app_result::FormatAppResult;
use crate::results::AppResult;
use crate::schema::file_uploads;

pub struct FileUploadRepository;

impl FileUploadRepository {
    #[allow(dead_code)]
    pub fn list_by_owner(
        &mut self,
        pool: &DBPool,
        owner_id: Uuid,
        q: QueryParams,
    ) -> AppResult<PaginationResult<FileUpload>> {
        file_uploads::table
            .filter(file_uploads::owner_id.eq(owner_id))
            .filter(file_uploads::deleted_at.is_null())
            .filter(file_uploads::orig_name.ilike(q.get_search_query_like()))
            .order_by(file_uploads::updated_at.asc())
            .paginate(q.get_page())
            .per_page(q.get_per_page())
            .load_and_count_pages::<FileUpload>(get_db_conn(pool).deref_mut())
            .into_app_result()
    }

    pub fn create(
        &mut self,
        pool: &DBPool,
        uploader_id: Uuid,
        form: FileUploadCreateForm,
    ) -> AppResult<FileUpload> {
        let model = FileUpload {
            file_upload_id: Uuid::new_v4(),
            uploader_id,
            owner_id: form.owner_id,
            owner_type: form.owner_type.to_string(),
            orig_name: form.orig_name,
            file_name: form.file_name,
            description: form.description,
            additional_info: form.additional_info,
            is_temp: form.is_temp,
            file_path: form.file_path,
            file_ext: form.file_ext,
            created_at: current_timestamp(),
            updated_at: current_timestamp(),
            deleted_at: None,
        };

        diesel::insert_into(file_uploads::dsl::file_uploads)
            .values(model)
            .get_result::<FileUpload>(get_db_conn(pool).deref_mut())
            .into_app_result()
    }

    pub fn delete(&mut self, pool: &DBPool, id: Uuid) -> AppResult<FileUpload> {
        let mut file_upload = self.find_by_id(pool, id)?;
        file_upload.deleted_at = Some(current_timestamp());
        file_upload
            .save_changes::<FileUpload>(get_db_conn(pool).deref_mut())
            .into_app_result()
    }

    pub fn find_by_id(&mut self, pool: &DBPool, id: Uuid) -> AppResult<FileUpload> {
        file_uploads::table
            .filter(file_uploads::file_upload_id.eq(id))
            .filter(file_uploads::deleted_at.is_null())
            .first::<FileUpload>(get_db_conn(pool).deref_mut())
            .required("file upload")
    }
}
