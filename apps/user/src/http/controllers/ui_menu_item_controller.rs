use actix_web::web::{block, Data, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, post, put, HttpRequest};
use uuid::Uuid;

use core::enums::permissions::Permissions;
use core::helpers::http::QueryParams;
use core::helpers::request::RequestHelper;
use core::helpers::DBPool;
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
async fn index(q: Query<QueryParams>, req: HttpRequest, pool: Data<DBPool>) -> HttpResult {
    req.verify_user_permission(Permissions::UiMenuItemList)?;
    block(move || UiMenuItemRepository.list_paginated(pool.get_ref(), q.0))
        .await
        .respond()
}

#[post("")]
async fn store(form: Json<CreateForm>, req: HttpRequest, pool: Data<DBPool>) -> HttpResult {
    req.verify_user_permission(Permissions::UiMenuItemCreate)?;
    let auth_id = req.auth_id();
    block(move || UiMenuItemService.create(pool.get_ref(), auth_id, form.0))
        .await
        .respond()
}

#[put("{id}")]
async fn update(
    id: Path<Uuid>,
    form: Json<CreateForm>,
    req: HttpRequest,
    pool: Data<DBPool>,
) -> HttpResult {
    req.verify_user_permission(Permissions::UiMenuItemUpdate)?;
    block(move || UiMenuItemService.update(pool.get_ref(), *id, form.0))
        .await
        .respond()
}

#[get("{id}")]
async fn show(id: Path<Uuid>, req: HttpRequest, pool: Data<DBPool>) -> HttpResult {
    req.verify_user_permission(Permissions::UiMenuItemRead)?;
    block(move || UiMenuItemRepository.find_by_id(pool.get_ref(), *id))
        .await
        .respond()
}

#[delete("{id}")]
async fn delete(id: Path<Uuid>, req: HttpRequest, pool: Data<DBPool>) -> HttpResult {
    req.verify_user_permission(Permissions::UiMenuItemDelete)?;
    block(move || UiMenuItemService.delete(pool.get_ref(), *id))
        .await
        .respond()
}
