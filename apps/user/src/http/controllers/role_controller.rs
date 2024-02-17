use actix_web::{delete, get, HttpRequest, patch, post, put};
use actix_web::web::{block, Json, Path, Query, ServiceConfig};
use uuid::Uuid;

use cosmic::enums::auth_permission::AuthPermission;
use cosmic::helpers::http::QueryParams;
use cosmic::helpers::request::RequestHelper;
use cosmic::models::role::RoleCreateForm;
use cosmic::models::user_permission::PermissionsParam;
use cosmic::repositories::role_permission_repository::RolePermissionRepository;
use cosmic::repositories::role_repository::RoleRepository;
use cosmic::repositories::user_repository::UserRepository;
use cosmic::results::http_result::ActixBlockingResultResponder;
use cosmic::results::HttpResult;
use cosmic::services::role_permission_service::RolePermissionService;
use cosmic::services::role_service::RoleService;

pub fn role_controller(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(store);
    cfg.service(show);
    cfg.service(update);
    cfg.service(users);
    cfg.service(activate);
    cfg.service(deactivate);
    cfg.service(assignable_permissions);
    cfg.service(add_permissions);
    cfg.service(permissions);
    cfg.service(role_find_by_name);
    cfg.service(permission_remove);
}

#[get("")]
async fn index(q: Query<QueryParams>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::RoleList)?;
        RoleRepository.list(ctx.database(), q.into_inner())
    })
    .await
    .respond()
}

#[post("")]
async fn store(req: HttpRequest, form: Json<RoleCreateForm>) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::RoleCreate)?;
        RoleService.create(ctx.database(), ctx.auth_id(), form.into_inner())
    })
    .await
    .respond()
}

#[get("{id}")]
async fn show(req: HttpRequest, id: Path<Uuid>) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::RoleRead)?;
        RoleRepository.find_by_id(ctx.database(), *id)
    })
    .await
    .respond()
}

#[put("{id}")]
async fn update(req: HttpRequest, id: Path<Uuid>, form: Json<RoleCreateForm>) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::RoleUpdate)?;
        RoleService.update(ctx.database(), *id, form.into_inner())
    })
    .await
    .respond()
}

#[get("{id}/users")]
async fn users(req: HttpRequest, id: Path<Uuid>, q: Query<QueryParams>) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::RoleUserList)?;
        UserRepository.list_by_role(ctx.database(), id.into_inner(), q.into_inner())
    })
    .await
    .respond()
}

#[patch("{id}/activate")]
async fn activate(req: HttpRequest, id: Path<Uuid>) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::RoleActivate)?;
        RoleService.activate(ctx.database(), *id)
    })
    .await
    .respond()
}

#[patch("{id}/deactivate")]
async fn deactivate(req: HttpRequest, id: Path<Uuid>) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::RoleDeactivate)?;
        RoleService.deactivate(ctx.database(), *id)
    })
    .await
    .respond()
}

#[get("{id}/permissions")]
async fn permissions(id: Path<Uuid>, q: Query<QueryParams>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::RoleList)?;
        RolePermissionRepository.list_paginated_by_role_id(ctx.database(), *id, q.into_inner())
    })
    .await
    .respond()
}

#[get("{id}/assignable-permissions")]
async fn assignable_permissions(id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::RoleList)?;
        RoleRepository.list_assignable_permissions(ctx.database(), *id)
    })
    .await
    .respond()
}

#[post("{id}/permissions")]
async fn add_permissions(
    user_id: Path<Uuid>,
    form: Json<PermissionsParam>,
    req: HttpRequest,
) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::RolePermissionCreate)?;

        let mut perms = vec![];
        let ids = form.into_inner().ids;
        for id in ids {
            let perm_result =
                RoleService.add_permission(ctx.database(), ctx.auth_id(), *user_id, id);

            if let Ok(perm) = perm_result {
                perms.push(perm);
            }
        }

        Ok(perms)
    })
    .await
    .respond()
}

#[delete("{id}/permissions/{pid}")]
async fn permission_remove(ids: Path<(Uuid, Uuid)>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::RoleList)?;
        RolePermissionService.remove(ctx.database(), ids.1)
    })
    .await
    .respond()
}

#[get("find-by-name/{name}")]
async fn role_find_by_name(name: Path<String>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::RoleList)?;
        RoleRepository.find_by_name(ctx.database(), name.into_inner())
    })
    .await
    .respond()
}
