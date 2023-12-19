use actix_web::web::{Data, Path, Query, ServiceConfig};
use actix_web::{get, patch, HttpRequest, HttpResponse};
use uuid::Uuid;

use core::app_state::AppState;
use core::helpers::http::QueryParams;
use core::helpers::request::RequestHelper;
use core::repositories::notification_repository::NotificationRepository;
use core::results::http_result::{ErroneousResponse, PaginationResponse};
use core::results::HttpResult;
use core::services::notification_service::NotificationService;

pub(crate) fn notification_controller(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(read);
    cfg.service(glance);
}

#[get("")]
async fn index(req: HttpRequest, q: Query<QueryParams>) -> HttpResponse {
    let db_pool = req.app_data::<Data<AppState>>().unwrap().database();
    NotificationRepository
        .list_paginated_by_user_id(db_pool, req.auth_id(), q.into_inner())
        .send_pagination()
}

#[patch("{id}/glance")]
async fn glance(req: HttpRequest, id: Path<Uuid>) -> HttpResult {
    let db_pool = req.app_data::<Data<AppState>>().unwrap().database();
    NotificationService
        .mark_as_glanced(db_pool, id.into_inner(), req.auth_id())
        .send_result()
}

#[patch("{id}/read")]
async fn read(req: HttpRequest, id: Path<Uuid>) -> HttpResult {
    let db_pool = req.app_data::<Data<AppState>>().unwrap().database();
    NotificationService
        .mark_as_read(db_pool, id.into_inner(), req.auth_id())
        .send_result()
}
