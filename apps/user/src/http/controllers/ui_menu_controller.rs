use actix_web::web::{block, Data, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, post, put, HttpRequest};
use uuid::Uuid;

use core::enums::auth_permission::AuthPermission;
use core::helpers::http::QueryParams;
use core::helpers::request::RequestHelper;
use core::helpers::DBPool;
use core::models::ui_menu::CreateForm;
use core::repositories::ui_menu_item_repository::UiMenuItemRepository;
use core::repositories::ui_menu_repository::UiMenuRepository;
use core::results::http_result::ActixBlockingResultResponder;
use core::results::HttpResult;
use core::services::ui_menu_service::UiMenuService;

pub fn ui_menu_controller(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(store);
    cfg.service(show);
    cfg.service(update);
    cfg.service(delete);
    cfg.service(menu_items);
}

#[get("")]
async fn index(q: Query<QueryParams>, req: HttpRequest, pool: Data<DBPool>) -> HttpResult {
    req.verify_user_permission(AuthPermission::UiMenuList)?;
    block(move || UiMenuRepository.list(pool.get_ref(), q.0))
        .await
        .respond()
}

#[post("")]
async fn store(form: Json<CreateForm>, req: HttpRequest, pool: Data<DBPool>) -> HttpResult {
    let auth_id = req.auth_id();
    req.verify_user_permission(AuthPermission::UiMenuCreate)?;
    block(move || UiMenuService.create(pool.get_ref(), auth_id, form.0))
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
    req.verify_user_permission(AuthPermission::UiMenuUpdate)?;
    block(move || UiMenuService.update(pool.get_ref(), *id, form.0))
        .await
        .respond()
}

#[get("{id}")]
async fn show(id: Path<Uuid>, req: HttpRequest, pool: Data<DBPool>) -> HttpResult {
    req.verify_user_permission(AuthPermission::UiMenuRead)?;
    block(move || UiMenuRepository.find_by_id(pool.get_ref(), *id))
        .await
        .respond()
}

#[delete("{id}")]
async fn delete(id: Path<Uuid>, req: HttpRequest, pool: Data<DBPool>) -> HttpResult {
    req.verify_user_permission(AuthPermission::UiMenuDelete)?;
    block(move || UiMenuService.delete(pool.get_ref(), *id))
        .await
        .respond()
}

#[get("{id}/items")]
async fn menu_items(
    id: Path<Uuid>,
    q: Query<QueryParams>,
    req: HttpRequest,
    pool: Data<DBPool>,
) -> HttpResult {
    req.verify_user_permission(AuthPermission::UiMenuRead)?;
    block(move || UiMenuItemRepository.list_by_menu_id(pool.get_ref(), *id, q.0))
        .await
        .respond()
}
