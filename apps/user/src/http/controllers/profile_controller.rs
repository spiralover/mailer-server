use actix_multipart::form::MultipartForm;
use actix_web::web::{Query, ServiceConfig};
use actix_web::{get, post, HttpRequest, HttpResponse};

use core::auth::check_permission;
use core::entities::Entities;
use core::helpers::http::{QueryParams, UploadForm};
use core::helpers::request::RequestHelper;
use core::helpers::string::string;
use core::models::file_upload::FileUploadData;
use core::permissions::Permissions;
use core::repositories::auth_attempt_repository::AuthAttemptRepository;
use core::results::http_result::{ErroneousResponse, PaginationResponse, StructResponse};
use core::results::HttpResult;
use core::services::file_upload_service::FileUploadService;
use core::services::user_service::UserService;

pub fn profile_controller(cfg: &mut ServiceConfig) {
    cfg.service(profile);
    cfg.service(auth_attempts);
    cfg.service(upload_passport);
}

#[get("")]
async fn profile(req: HttpRequest) -> HttpResponse {
    req.auth_user().into_sharable().send_response()
}

#[get("auth-attempts")]
async fn auth_attempts(req: HttpRequest, q: Query<QueryParams>) -> HttpResponse {
    AuthAttemptRepository
        .list_by_email(req.get_db_pool(), req.auth_user().email, q.into_inner())
        .send_pagination()
}

#[post("passport")]
async fn upload_passport(req: HttpRequest, form: MultipartForm<UploadForm>) -> HttpResult {
    let app = req.get_app_state();

    check_permission(req.to_owned(), Permissions::UserMyProfileUploadPassport)?;

    let auth_user = req.auth_user();

    let file = FileUploadService.upload(
        app,
        form.into_inner().file,
        FileUploadData {
            uploader_id: auth_user.user_id,
            owner_id: auth_user.user_id,
            owner_type: Entities::User,
            description: Some(string("profile picture")),
            additional_info: None,
            is_temp: false,
        },
    )?;

    UserService
        .change_profile_picture(app.get_db_pool(), auth_user, file.file_path)
        .map(|u| u.into_sharable())
        .send_result()
}
