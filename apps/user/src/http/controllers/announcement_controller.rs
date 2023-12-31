use actix_web::web::{block, Data, Json, Path, Query, ServiceConfig};
use actix_web::{get, post, HttpRequest};
use uuid::Uuid;

use core::app_state::AppState;
use core::enums::auth_permission::AuthPermission;
use core::helpers::auth::check_permission;
use core::helpers::http::QueryParams;
use core::helpers::request::RequestHelper;
use core::helpers::DBPool;
use core::models::announcement::AnnouncementCreateForm;
use core::repositories::announcement_repository::AnnouncementRepository;
use core::results::http_result::ActixBlockingResultResponder;
use core::results::HttpResult;
use core::services::announcement_service::AnnouncementService;

pub fn announcement_controller(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(send);
    cfg.service(show);
}

#[get("")]
async fn index(req: HttpRequest, q: Query<QueryParams>) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::AnnouncementList)?;
        AnnouncementRepository.list(ctx.database(), q.0)
    })
    .await
    .respond()
}

#[post("")]
async fn send(
    req: HttpRequest,
    form: Json<AnnouncementCreateForm>,
    app: Data<AppState>,
) -> HttpResult {
    req.verify_user_permission(AuthPermission::AnnouncementSend)?;
    let auth_id = req.auth_id();
    block(move || AnnouncementService.send(app.into_inner(), auth_id, form.0))
        .await
        .respond()
}

#[get("{id}")]
async fn show(req: HttpRequest, pool: Data<DBPool>, id: Path<Uuid>) -> HttpResult {
    check_permission(req.to_owned(), AuthPermission::AnnouncementRead)?;
    block(move || AnnouncementRepository.find_by_id(pool.get_ref(), *id))
        .await
        .respond()
}
