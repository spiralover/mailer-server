use actix_web::web::{block, Path, Query, ServiceConfig};
use actix_web::{get, patch, HttpRequest};
use uuid::Uuid;

use core::helpers::http::QueryParams;
use core::helpers::request::RequestHelper;
use core::repositories::notification_repository::NotificationRepository;
use core::results::http_result::ActixBlockingResultResponder;
use core::results::HttpResult;
use core::services::notification_service::NotificationService;

pub(crate) fn notification_controller(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(read);
    cfg.service(glance);
}

#[get("")]
async fn index(req: HttpRequest, q: Query<QueryParams>) -> HttpResult {
    let ctx = req.context();
    block(move || {
        NotificationRepository.list_paginated_by_user_id(
            ctx.database(),
            ctx.auth_id(),
            q.into_inner(),
        )
    })
    .await
    .respond()
}

#[patch("{id}/glance")]
async fn glance(req: HttpRequest, id: Path<Uuid>) -> HttpResult {
    let ctx = req.context();
    block(move || {
        NotificationService.mark_as_glanced(ctx.database(), id.into_inner(), ctx.auth_id())
    })
    .await
    .respond()
}

#[patch("{id}/read")]
async fn read(req: HttpRequest, id: Path<Uuid>) -> HttpResult {
    let ctx = req.context();
    block(move || NotificationService.mark_as_read(ctx.database(), id.into_inner(), ctx.auth_id()))
        .await
        .respond()
}
