use actix_multipart::form::MultipartForm;
use actix_web::web::ServiceConfig;
use actix_web::{post, HttpRequest};

use core::auth::check_permission;
use core::entities::Entities;
use core::helpers::http::UploadForm;
use core::helpers::request::RequestHelper;
use core::helpers::string::string;
use core::models::file_upload::FileUploadData;
use core::permissions::Permissions;
use core::results::http_result::ErroneousResponse;
use core::results::HttpResult;
use core::services::file_upload_service::FileUploadService;

pub fn misc_controller(cfg: &mut ServiceConfig) {
    cfg.service(upload_temp_file);
}

#[post("temp-file")]
async fn upload_temp_file(req: HttpRequest, form: MultipartForm<UploadForm>) -> HttpResult {
    let app = req.get_app_state();

    check_permission(req.to_owned(), Permissions::MiscUploadTempFile)?;

    let auth_user = req.auth_user();

    FileUploadService
        .upload(
            app,
            form.into_inner().file,
            FileUploadData {
                uploader_id: auth_user.user_id,
                owner_id: auth_user.user_id,
                owner_type: Entities::Temp,
                description: Some(string("temporary file")),
                additional_info: None,
                is_temp: true,
            },
        )
        .send_result()
}
