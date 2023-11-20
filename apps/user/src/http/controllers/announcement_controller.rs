use actix_web::web::{Data, Json, Path, Query, ServiceConfig};
use actix_web::{get, post, HttpRequest};
use uuid::Uuid;

use core::app_state::AppState;
use core::auth::check_permission;
use core::helpers::http::QueryParams;
use core::helpers::request::RequestHelper;
use core::models::announcement::AnnouncementCreateForm;
use core::permissions::Permissions;
use core::repositories::announcement_repository::AnnouncementRepository;
use core::results::http_result::{ErroneousResponse, PaginationResponse};
use core::results::HttpResult;
use core::services::announcement_service::AnnouncementService;

pub fn announcement_controller(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(send);
    cfg.service(show);
}

#[get("")]
async fn index(req: HttpRequest, q: Query<QueryParams>) -> HttpResult {
    check_permission(req.to_owned(), Permissions::AnnouncementList)?;
    let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
    AnnouncementRepository
        .list(db_pool, q.into_inner())
        .send_pagination_result()
}

#[post("")]
async fn send(req: HttpRequest, form: Json<AnnouncementCreateForm>) -> HttpResult {
    check_permission(req.to_owned(), Permissions::AnnouncementSend)?;
    let app = req.app_data::<Data<AppState>>().unwrap().get_ref();
    let auth_id = req.auth_id();
    AnnouncementService
        .send(app, auth_id, form.to_owned())
        .await
        .send_result()
}

#[get("{id}")]
async fn show(req: HttpRequest, id: Path<Uuid>) -> HttpResult {
    check_permission(req.to_owned(), Permissions::AnnouncementRead)?;
    let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
    AnnouncementRepository
        .find_by_id(db_pool, *id)
        .send_result()
}
