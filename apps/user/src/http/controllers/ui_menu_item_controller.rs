use actix_web::web::{block, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, post, put, HttpRequest};
use uuid::Uuid;

use core::enums::auth_permission::AuthPermission;
use core::helpers::http::QueryParams;
use core::helpers::request::RequestHelper;
use core::models::ui_menu_item::CreateForm;
use core::repositories::ui_menu_item_repository::UiMenuItemRepository;
use core::results::http_result::ActixBlockingResultResponder;
use core::results::HttpResult;
use core::services::ui_menu_item_service::UiMenuItemService;

pub fn ui_menu_item_controller(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(store);
    cfg.service(show);
    cfg.service(update);
    cfg.service(delete);
}

#[get("")]
async fn index(q: Query<QueryParams>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::UiMenuItemList)?;
        UiMenuItemRepository.list_paginated(ctx.database(), q.0)
    })
    .await
    .respond()
}

#[post("")]
async fn store(form: Json<CreateForm>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::UiMenuItemCreate)?;
        UiMenuItemService.create(ctx.database(), ctx.auth_id(), form.0)
    })
    .await
    .respond()
}

#[put("{id}")]
async fn update(id: Path<Uuid>, form: Json<CreateForm>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::UiMenuItemUpdate)?;
        UiMenuItemService.update(ctx.database(), *id, form.0)
    })
    .await
    .respond()
}

#[get("{id}")]
async fn show(id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::UiMenuItemRead)?;
        UiMenuItemRepository.find_by_id(ctx.database(), *id)
    })
    .await
    .respond()
}

#[delete("{id}")]
async fn delete(id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::UiMenuItemDelete)?;
        UiMenuItemService.delete(ctx.database(), *id)
    })
    .await
    .respond()
}
