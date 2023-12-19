use actix_multipart::form::MultipartForm;
use actix_web::web::{block, Data, ServiceConfig};
use actix_web::{post, HttpRequest};

use crate::app_state::AppState;
use crate::enums::entities::Entities;
use crate::enums::permissions::Permissions;
use crate::helpers::http::UploadForm;
use crate::helpers::request::RequestHelper;
use crate::helpers::string::string;
use crate::models::file_upload::FileUploadData;
use crate::results::http_result::ActixBlockingResultResponder;
use crate::results::HttpResult;
use crate::services::file_upload_service::FileUploadService;

pub fn misc_controller(cfg: &mut ServiceConfig) {
    cfg.service(upload_temp_file);
}

#[post("temp-file")]
async fn upload_temp_file(
    req: HttpRequest,
    app: Data<AppState>,
    form: MultipartForm<UploadForm>,
) -> HttpResult {
    req.verify_user_permission(Permissions::MiscUploadTempFile)?;
    let auth_user = req.auth_user();

    block(move || {
        FileUploadService.upload(
            app.into_inner(),
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
    })
    .await
    .respond()
}
