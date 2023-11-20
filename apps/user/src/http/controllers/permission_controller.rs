use actix_web::web::{Data, Path, Query, ServiceConfig};
use actix_web::{get, HttpRequest};

use core::app_state::AppState;
use core::auth::has_permission;
use core::helpers::http::QueryParams;
use core::permissions::Permissions;
use core::repositories::permission_repository::PermissionRepository;
use core::results::http_result::{ErroneousResponse, PaginationResponse};
use core::results::HttpResult;

pub fn permission_controller(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(find_by_name);
}

#[get("")]
async fn index(q: Query<QueryParams>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::PermissionList, || {
        let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
        PermissionRepository
            .list(db_pool, q.into_inner())
            .send_pagination_result()
    })
}

#[get("find-by-name/{name}")]
async fn find_by_name(name: Path<String>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::PermissionList, || {
        let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
        PermissionRepository
            .find_by_name(db_pool, name.into_inner())
            .send_result()
    })
}
