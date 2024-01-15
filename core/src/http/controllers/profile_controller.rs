use actix_multipart::form::MultipartForm;
use actix_web::web::{block, Data, Query, ServiceConfig};
use actix_web::{get, post, HttpRequest};

use crate::enums::auth_permission::AuthPermission;
use crate::enums::entities::Entities;
use crate::helpers::http::{QueryParams, UploadForm};
use crate::helpers::request::RequestHelper;
use crate::helpers::string::string;
use crate::helpers::DBPool;
use crate::models::file_upload::FileUploadData;
use crate::repositories::auth_attempt_repository::AuthAttemptRepository;
use crate::repositories::user_repository::UserRepository;
use crate::results::http_result::ActixBlockingResultResponder;
use crate::results::HttpResult;
use crate::services::file_upload_service::FileUploadService;
use crate::services::user_service::UserService;

pub fn profile_controller(cfg: &mut ServiceConfig) {
    cfg.service(profile);
    cfg.service(auth_attempts);
    cfg.service(upload_passport);
}

#[get("")]
async fn profile(req: HttpRequest, pool: Data<DBPool>) -> HttpResult {
    let auth_id = req.auth_id();
    block(move || {
        UserRepository
            .find_by_id(pool.get_ref(), auth_id)
            .map(|u| u.into_sharable())
    })
    .await
    .respond()
}

#[get("auth-attempts")]
async fn auth_attempts(req: HttpRequest, pool: Data<DBPool>, q: Query<QueryParams>) -> HttpResult {
    let auth_user = req.auth_user();
    block(move || AuthAttemptRepository.list_by_email(pool.get_ref(), auth_user.email, q.0))
        .await
        .respond()
}

#[post("passport")]
async fn upload_passport(req: HttpRequest, form: MultipartForm<UploadForm>) -> HttpResult {
    let ctx = req.context();

    block(move || {
        ctx.verify_user_permission(AuthPermission::UserMyProfileUploadPassport)?;
        let auth_user = UserRepository.find_by_id(ctx.database(), ctx.auth_id)?;

        let file = FileUploadService.upload(
            ctx.app(),
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
            .change_profile_picture(ctx.database(), auth_user, file.file_path)
            .map(|u| u.into_sharable())
    })
    .await
    .respond()
}
