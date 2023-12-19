use actix_web::web::{block, Data, Path, Query, ServiceConfig};
use actix_web::{get, HttpRequest};

use core::enums::permissions::Permissions;
use core::helpers::http::QueryParams;
use core::helpers::request::RequestHelper;
use core::helpers::DBPool;
use core::repositories::permission_repository::PermissionRepository;
use core::results::http_result::ActixBlockingResultResponder;
use core::results::HttpResult;

pub fn permission_controller(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(find_by_name);
}

#[get("")]
async fn index(q: Query<QueryParams>, req: HttpRequest, pool: Data<DBPool>) -> HttpResult {
    req.verify_user_permission(Permissions::PermissionList)?;
    block(move || PermissionRepository.list(pool.get_ref(), q.into_inner()))
        .await
        .respond()
}

#[get("find-by-name/{name}")]
async fn find_by_name(name: Path<String>, req: HttpRequest, pool: Data<DBPool>) -> HttpResult {
    req.verify_user_permission(Permissions::PermissionList)?;
    block(move || PermissionRepository.find_by_name(pool.get_ref(), name.into_inner()))
        .await
        .respond()
}
