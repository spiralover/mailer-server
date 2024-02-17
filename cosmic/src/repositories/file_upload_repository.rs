use diesel::{ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl, SaveChangesDsl};
use uuid::Uuid;

use crate::helpers::db::{DatabaseConnectionHelper, OptionalResult};
use crate::helpers::db_pagination::{PageData, Paginate};
use crate::helpers::http::QueryParams;
use crate::helpers::time::current_timestamp;
use crate::helpers::DBPool;
use crate::models::file_upload::{FileUpload, FileUploadCreateForm};
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
    ) -> AppResult<PageData<FileUpload>> {
        file_uploads::table
            .filter(file_uploads::owner_id.eq(owner_id))
            .filter(file_uploads::deleted_at.is_null())
            .filter(file_uploads::orig_name.ilike(q.get_search_query_like()))
            .order_by(file_uploads::updated_at.asc())
            .paginate(q.get_page())
            .per_page(q.get_per_page())
            .load_and_count_pages::<FileUpload>(&mut pool.conn())
            .into_app_result()
    }

    pub fn create(
        &mut self,
        pool: &DBPool,
        uploader_id: Uuid,
        form: FileUploadCreateForm,
    ) -> AppResult<FileUpload> {
        diesel::insert_into(file_uploads::dsl::file_uploads)
            .values(FileUpload {
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
            })
            .get_result::<FileUpload>(&mut pool.conn())
            .into_app_result()
    }

    pub fn delete(&mut self, pool: &DBPool, id: Uuid) -> AppResult<FileUpload> {
        let mut file_upload = self.find_by_id(pool, id)?;
        file_upload.deleted_at = Some(current_timestamp());
        file_upload
            .save_changes::<FileUpload>(&mut pool.conn())
            .into_app_result()
    }

    pub fn find_by_id(&mut self, pool: &DBPool, id: Uuid) -> AppResult<FileUpload> {
        file_uploads::table
            .filter(file_uploads::file_upload_id.eq(id))
            .filter(file_uploads::deleted_at.is_null())
            .first::<FileUpload>(&mut pool.conn())
            .required("file upload")
    }
}
