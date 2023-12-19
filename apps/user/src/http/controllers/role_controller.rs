use actix_web::web::{block, Data, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, patch, post, put, HttpRequest};
use uuid::Uuid;

use core::enums::permissions::Permissions;
use core::helpers::http::QueryParams;
use core::helpers::request::RequestHelper;
use core::helpers::DBPool;
use core::models::role::RoleCreateForm;
use core::models::user_permission::PermissionsParam;
use core::repositories::role_permission_repository::RolePermissionRepository;
use core::repositories::role_repository::RoleRepository;
use core::repositories::user_repository::UserRepository;
use core::results::http_result::ActixBlockingResultResponder;
use core::results::HttpResult;
use core::services::role_permission_service::RolePermissionService;
use core::services::role_service::RoleService;

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
async fn index(q: Query<QueryParams>, req: HttpRequest, pool: Data<DBPool>) -> HttpResult {
    req.verify_user_permission(Permissions::RoleList)?;
    block(move || RoleRepository.list(pool.get_ref(), q.into_inner()))
        .await
        .respond()
}

#[post("")]
async fn store(req: HttpRequest, pool: Data<DBPool>, form: Json<RoleCreateForm>) -> HttpResult {
    let auth_id = req.auth_id();
    req.verify_user_permission(Permissions::RoleCreate)?;
    block(move || RoleService.create(pool.get_ref(), auth_id, form.into_inner()))
        .await
        .respond()
}

#[get("{id}")]
async fn show(req: HttpRequest, pool: Data<DBPool>, id: Path<Uuid>) -> HttpResult {
    req.verify_user_permission(Permissions::RoleRead)?;
    block(move || RoleRepository.find_by_id(pool.get_ref(), *id))
        .await
        .respond()
}

#[put("{id}")]
async fn update(
    req: HttpRequest,
    pool: Data<DBPool>,
    id: Path<Uuid>,
    form: Json<RoleCreateForm>,
) -> HttpResult {
    req.verify_user_permission(Permissions::RoleUpdate)?;
    block(move || RoleService.update(pool.get_ref(), *id, form.into_inner()))
        .await
        .respond()
}

#[get("{id}/users")]
async fn users(
    req: HttpRequest,
    pool: Data<DBPool>,
    id: Path<Uuid>,
    q: Query<QueryParams>,
) -> HttpResult {
    req.verify_user_permission(Permissions::RoleUserList)?;
    block(move || UserRepository.list_by_role(pool.get_ref(), id.into_inner(), q.into_inner()))
        .await
        .respond()
}

#[patch("{id}/activate")]
async fn activate(req: HttpRequest, pool: Data<DBPool>, id: Path<Uuid>) -> HttpResult {
    req.verify_user_permission(Permissions::RoleActivate)?;
    block(move || RoleService.activate(pool.get_ref(), *id))
        .await
        .respond()
}

#[patch("{id}/deactivate")]
async fn deactivate(req: HttpRequest, pool: Data<DBPool>, id: Path<Uuid>) -> HttpResult {
    req.verify_user_permission(Permissions::RoleDeactivate)?;
    block(move || RoleService.deactivate(pool.get_ref(), *id))
        .await
        .respond()
}

#[get("{id}/permissions")]
async fn permissions(
    id: Path<Uuid>,
    q: Query<QueryParams>,
    req: HttpRequest,
    pool: Data<DBPool>,
) -> HttpResult {
    req.verify_user_permission(Permissions::RoleList)?;
    block(move || {
        RolePermissionRepository.list_paginated_by_role_id(pool.get_ref(), *id, q.into_inner())
    })
        .await
        .respond()
}

#[get("{id}/assignable-permissions")]
async fn assignable_permissions(
    id: Path<Uuid>,
    req: HttpRequest,
    pool: Data<DBPool>,
) -> HttpResult {
    req.verify_user_permission(Permissions::RoleList)?;
    block(move || RoleRepository.list_assignable_permissions(pool.get_ref(), *id))
        .await
        .respond()
}

#[post("{id}/permissions")]
async fn add_permissions(
    user_id: Path<Uuid>,
    form: Json<PermissionsParam>,
    req: HttpRequest,
    pool: Data<DBPool>,
) -> HttpResult {
    let auth_id = req.auth_id();
    req.verify_user_permission(Permissions::RolePermissionCreate)?;
    block(move || {
        let mut perms = vec![];
        let ids = form.into_inner().ids;
        for id in ids {
            let perm_result = RoleService.add_permission(pool.get_ref(), auth_id, *user_id, id);

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
async fn permission_remove(
    ids: Path<(Uuid, Uuid)>,
    req: HttpRequest,
    pool: Data<DBPool>,
) -> HttpResult {
    req.verify_user_permission(Permissions::RoleList)?;
    block(move || RolePermissionService.remove(pool.get_ref(), ids.1))
        .await
        .respond()
}

#[get("find-by-name/{name}")]
async fn role_find_by_name(name: Path<String>, req: HttpRequest, pool: Data<DBPool>) -> HttpResult {
    req.verify_user_permission(Permissions::RoleList)?;
    block(move || RoleRepository.find_by_name(pool.get_ref(), name.into_inner()))
        .await
        .respond()
}
