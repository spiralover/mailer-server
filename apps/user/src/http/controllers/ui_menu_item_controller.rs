use actix_web::web::{Data, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, post, put, HttpRequest};
use uuid::Uuid;

use core::app_state::AppState;
use core::auth::has_permission;
use core::helpers::http::QueryParams;
use core::helpers::request::RequestHelper;
use core::models::ui_menu_item::CreateForm;
use core::permissions::Permissions;
use core::repositories::ui_menu_item_repository::UiMenuItemRepository;
use core::results::http_result::{ErroneousResponse, PaginationResponse};
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
    has_permission(req.to_owned(), Permissions::UiMenuItemList, || {
        let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
        UiMenuItemRepository
            .list_paginated(db_pool, q.into_inner())
            .send_pagination_result()
    })
}

#[post("")]
async fn store(form: Json<CreateForm>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::UiMenuItemCreate, || {
        let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
        UiMenuItemService
            .create(db_pool, req.auth_id(), form.into_inner())
            .send_result()
    })
}

#[put("{id}")]
async fn update(id: Path<Uuid>, form: Json<CreateForm>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::UiMenuItemUpdate, || {
        let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
        UiMenuItemService
            .update(db_pool, *id, form.into_inner())
            .send_result()
    })
}

#[get("{id}")]
async fn show(id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::UiMenuItemRead, || {
        let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
        UiMenuItemRepository.find_by_id(db_pool, *id).send_result()
    })
}

#[delete("{id}")]
async fn delete(id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::UiMenuItemDelete, || {
        let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
        UiMenuItemService.delete(db_pool, *id).send_result()
    })
}
