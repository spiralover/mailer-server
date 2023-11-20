use actix_web::web::{Data, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, patch, post, put, HttpRequest};
use uuid::Uuid;

use core::app_state::AppState;
use core::auth::has_permission;
use core::helpers::http::QueryParams;
use core::helpers::request::RequestHelper;
use core::models::role::RoleCreateForm;
use core::models::user_permission::PermissionsParam;
use core::permissions::Permissions;
use core::repositories::role_permission_repository::RolePermissionRepository;
use core::repositories::role_repository::RoleRepository;
use core::repositories::user_repository::UserRepository;
use core::results::http_result::{ErroneousResponse, PaginationResponse, StructResponse};
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
async fn index(q: Query<QueryParams>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::RoleList, || {
        let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
        RoleRepository
            .list(db_pool, q.into_inner())
            .send_pagination_result()
    })
}

#[post("")]
async fn store(req: HttpRequest, form: Json<RoleCreateForm>) -> HttpResult {
    has_permission(req.to_owned(), Permissions::RoleCreate, || {
        let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
        RoleService
            .create(db_pool, req.auth_id(), form.into_inner())
            .send_result()
    })
}

#[get("{id}")]
async fn show(req: HttpRequest, id: Path<Uuid>) -> HttpResult {
    has_permission(req.to_owned(), Permissions::RoleRead, || {
        let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
        RoleRepository.find_by_id(db_pool, *id).send_result()
    })
}

#[put("{id}")]
async fn update(req: HttpRequest, id: Path<Uuid>, form: Json<RoleCreateForm>) -> HttpResult {
    has_permission(req.to_owned(), Permissions::RoleUpdate, || {
        let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
        RoleService
            .update(db_pool, *id, form.into_inner())
            .send_result()
    })
}

#[get("{id}/users")]
async fn users(req: HttpRequest, id: Path<Uuid>, q: Query<QueryParams>) -> HttpResult {
    has_permission(req.to_owned(), Permissions::RoleUserList, || {
        let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
        UserRepository
            .list_by_role(db_pool, id.into_inner(), q.into_inner())
            .send_pagination_result()
    })
}

#[patch("{id}/activate")]
async fn activate(req: HttpRequest, id: Path<Uuid>) -> HttpResult {
    has_permission(req.to_owned(), Permissions::RoleActivate, || {
        let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
        RoleService.activate(db_pool, *id).send_result()
    })
}

#[patch("{id}/deactivate")]
async fn deactivate(req: HttpRequest, id: Path<Uuid>) -> HttpResult {
    has_permission(req.to_owned(), Permissions::RoleDeactivate, || {
        let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
        RoleService.deactivate(db_pool, *id).send_result()
    })
}

#[get("{id}/permissions")]
async fn permissions(id: Path<Uuid>, q: Query<QueryParams>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::RoleList, || {
        let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
        RolePermissionRepository
            .list_paginated_by_role_id(db_pool, *id, q.into_inner())
            .send_pagination_result()
    })
}

#[get("{id}/assignable-permissions")]
async fn assignable_permissions(id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::RoleList, || {
        let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
        RoleRepository
            .list_assignable_permissions(db_pool, *id)
            .send_result()
    })
}

#[post("{id}/permissions")]
async fn add_permissions(
    user_id: Path<Uuid>,
    form: Json<PermissionsParam>,
    req: HttpRequest,
) -> HttpResult {
    has_permission(req.to_owned(), Permissions::RolePermissionCreate, || {
        let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();

        let mut perms = vec![];
        let ids = form.into_inner().ids;
        for id in ids {
            let perm_result = RoleService.add_permission(db_pool, req.auth_id(), *user_id, id);

            if let Ok(perm) = perm_result {
                perms.push(perm);
            }
        }

        perms.send_struct_result()
    })
}

#[delete("{id}/permissions/{pid}")]
async fn permission_remove(ids: Path<(Uuid, Uuid)>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::RoleList, || {
        let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
        RolePermissionService.remove(db_pool, ids.1).send_result()
    })
}

#[get("find-by-name/{name}")]
async fn role_find_by_name(name: Path<String>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::RoleList, || {
        let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
        RoleRepository
            .find_by_name(db_pool, name.into_inner())
            .send_result()
    })
}
