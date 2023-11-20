use actix_web::web::{Data, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, post, put, HttpRequest};
use uuid::Uuid;

use core::app_state::AppState;
use core::auth::has_permission;
use core::helpers::http::QueryParams;
use core::helpers::request::RequestHelper;
use core::models::ui_menu::CreateForm;
use core::permissions::Permissions;
use core::repositories::ui_menu_item_repository::UiMenuItemRepository;
use core::repositories::ui_menu_repository::UiMenuRepository;
use core::results::http_result::{ErroneousResponse, PaginationResponse};
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
async fn index(q: Query<QueryParams>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::UiMenuList, || {
        let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
        UiMenuRepository
            .list(db_pool, q.into_inner())
            .send_pagination_result()
    })
}

#[post("")]
async fn store(form: Json<CreateForm>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::UiMenuCreate, || {
        let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
        UiMenuService
            .create(db_pool, req.auth_id(), form.into_inner())
            .send_result()
    })
}

#[put("{id}")]
async fn update(id: Path<Uuid>, form: Json<CreateForm>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::UiMenuUpdate, || {
        let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
        UiMenuService
            .update(db_pool, *id, form.into_inner())
            .send_result()
    })
}

#[get("{id}")]
async fn show(id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::UiMenuRead, || {
        let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
        UiMenuRepository.find_by_id(db_pool, *id).send_result()
    })
}

#[delete("{id}")]
async fn delete(id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    req.verify_user_permission(Permissions::UiMenuDelete)?;
    let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
    UiMenuService.delete(db_pool, *id).send_result()
}

#[get("{id}/items")]
async fn menu_items(id: Path<Uuid>, q: Query<QueryParams>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::UiMenuRead, || {
        let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
        UiMenuItemRepository
            .list_by_menu_id(db_pool, *id, q.into_inner())
            .send_pagination_result()
    })
}
