use actix_web::web::{block, Path, Query, ServiceConfig};
use actix_web::{get, HttpRequest};

use cosmic::enums::auth_permission::AuthPermission;
use cosmic::helpers::http::QueryParams;
use cosmic::helpers::request::RequestHelper;
use cosmic::repositories::permission_repository::PermissionRepository;
use cosmic::results::http_result::ActixBlockingResultResponder;
use cosmic::results::HttpResult;

pub fn permission_controller(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(find_by_name);
}

#[get("")]
async fn index(q: Query<QueryParams>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::PermissionList)?;
        PermissionRepository.list(ctx.database(), q.into_inner())
    })
    .await
    .respond()
}

#[get("find-by-name/{name}")]
async fn find_by_name(name: Path<String>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::PermissionList)?;
        PermissionRepository.find_by_name(ctx.database(), name.into_inner())
    })
    .await
    .respond()
}
