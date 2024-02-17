use actix_web::web::{block, Data, Json, Path, Query, ServiceConfig};
use actix_web::{get, post, HttpRequest};
use uuid::Uuid;

use cosmic::app_state::AppState;
use cosmic::enums::auth_permission::AuthPermission;
use cosmic::helpers::auth::check_permission;
use cosmic::helpers::http::QueryParams;
use cosmic::helpers::request::RequestHelper;
use cosmic::helpers::DBPool;
use cosmic::models::announcement::AnnouncementCreateForm;
use cosmic::repositories::announcement_repository::AnnouncementRepository;
use cosmic::results::http_result::ActixBlockingResultResponder;
use cosmic::results::HttpResult;
use cosmic::services::announcement_service::AnnouncementService;

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
