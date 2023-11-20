use actix_multipart::form::MultipartForm;
use actix_web::web::{Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, patch, post, put, HttpRequest};
use log::error;
use uuid::Uuid;
use validator::Validate;

use core::auth::{check_permission, has_permission};
use core::entities::Entities;
use core::enums::app_message::AppMessage::SuccessMessage;
use core::helpers::http::{QueryParams, UploadForm};
use core::helpers::request::RequestHelper;
use core::helpers::string::string;
use core::models::file_upload::FileUploadData;
use core::models::role::RoleParam;
use core::models::user::{PasswordForm, User, UserRegisterForm, UserStatus, UserUpdateForm};
use core::models::user_permission::PermissionsParam;
use core::models::user_ui_menu_item::{
    MenuItemCreateDto, UserUiMenuItem, UserUiMenuItemCreateForm,
};
use core::permissions::Permissions;
use core::repositories::auth_attempt_repository::AuthAttemptRepository;
use core::repositories::role_repository::RoleRepository;
use core::repositories::user_permission_repository::UserPermissionRepository;
use core::repositories::user_repository::UserRepository;
use core::repositories::user_role_repository::UserRoleRepository;
use core::repositories::user_ui_menu_item_repository::UserUiMenuItemRepository;
use core::results::http_result::{ErroneousResponse, PaginationResponse, StructResponse};
use core::results::HttpResult;
use core::services::file_upload_service::FileUploadService;
use core::services::role_service::RoleService;
use core::services::user_service::UserService;
use core::services::user_ui_menu_item_service::UserUiMenuItemService;

pub fn user_controller(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(store);
    cfg.service(show);
    cfg.service(update);
    cfg.service(upload_passport);
    cfg.service(activate);
    cfg.service(deactivate);
    cfg.service(auth_attempts);
    cfg.service(change_password);

    cfg.service(roles);
    cfg.service(assignable_roles);
    cfg.service(assign_role);
    cfg.service(un_assign_role);

    cfg.service(individual_permissions);
    cfg.service(assignable_permissions);
    cfg.service(add_permission);
    cfg.service(remove_permission);

    cfg.service(menus);
    cfg.service(menu_items);
    cfg.service(assignable_menu_items);
    cfg.service(add_menu_item);
    cfg.service(remove_menu_item);
}

#[get("")]
async fn index(q: Query<QueryParams>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::UserList, || {
        let db_pool = req.get_db_pool();
        UserRepository
            .list(db_pool, q.into_inner())
            .map(|mut paginated| {
                let users: Vec<User> = paginated
                    .records
                    .iter()
                    .map(|user| {
                        let mut user = user.clone();
                        user.password = string("");
                        user.clone()
                    })
                    .collect();

                paginated.records = users;
                paginated
            })
            .send_pagination_result()
    })
}

#[post("")]
async fn store(form: Json<UserRegisterForm>, req: HttpRequest) -> HttpResult {
    let app = req.get_app_state();
    form.validate()?;
    req.verify_user_permission(Permissions::UserCreate)?;

    let db_pool = app.get_db_pool();
    let default_role_id = RoleRepository.get_default_role_id(db_pool);
    UserService
        .create(
            app,
            default_role_id,
            form.into_inner(),
            Some(UserStatus::Active),
        )
        .await
        .map(|u| u.into_sharable())
        .send_result()
}

#[get("{id}")]
async fn show(id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::UserRead, || {
        let db_pool = req.get_db_pool();
        UserService
            .get_profile(db_pool, id.into_inner())?
            .send_struct_result()
    })
}

#[put("{id}")]
async fn update(form: Json<UserUpdateForm>, id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::UserUpdate, || {
        let db_pool = req.get_db_pool();
        UserService.update(db_pool, *id, form.into_inner())?;
        UserService
            .get_profile(db_pool, id.into_inner())
            .send_result()
    })
}

#[patch("{id}/activate")]
async fn activate(id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::UserActivate, || {
        let db_pool = req.get_db_pool();
        UserService
            .activate(db_pool, id.into_inner())
            .map(|u| u.into_sharable())
            .send_result()
    })
}

#[patch("{id}/deactivate")]
async fn deactivate(id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::UserDeactivate, || {
        let db_pool = req.get_db_pool();
        UserService
            .deactivate(db_pool, id.into_inner())
            .map(|u| u.into_sharable())
            .send_result()
    })
}

#[patch("{id}/change-password")]
async fn change_password(id: Path<Uuid>, req: HttpRequest, form: Json<PasswordForm>) -> HttpResult {
    let form = form.into_inner();
    form.validate()?;

    has_permission(req.to_owned(), Permissions::UserChangePassword, || {
        let db_pool = req.get_db_pool();
        UserService
            .change_password(db_pool, id.into_inner(), form.password)
            .map(|u| u.into_sharable())
            .send_result()
    })
}

#[get("{id}/auth-attempts")]
async fn auth_attempts(id: Path<Uuid>, q: Query<QueryParams>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::UserAuthAttemptList, || {
        let db_pool = req.get_db_pool();
        let email = UserRepository.fetch_email(db_pool, id.into_inner())?;
        AuthAttemptRepository
            .list_by_email(db_pool, email, q.into_inner())
            .send_pagination_result()
    })
}

#[get("{id}/roles")]
async fn roles(id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::UserRoleList, || {
        let db_pool = req.get_db_pool();
        UserRoleRepository
            .list_paginated_by_user_id(db_pool, id.into_inner())
            .send_pagination_result()
    })
}

#[post("{id}/roles")]
async fn assign_role(id: Path<Uuid>, form: Json<RoleParam>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::UserRoleAssign, || {
        let db_pool = req.get_db_pool();
        let role_id = form.into_inner().role_id;
        RoleService
            .assign_role_to_user(db_pool, req.auth_id(), role_id, id.into_inner())
            .send_result()
    })
}

#[get("{id}/assignable-roles")]
async fn assignable_roles(id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::UserRoleList, || {
        let db_pool = req.get_db_pool();
        UserRoleRepository
            .list_assignable(db_pool, id.into_inner())
            .send_result()
    })
}

#[delete("{id}/roles/{rid}")]
async fn un_assign_role(path: Path<(Uuid, Uuid)>, req: HttpRequest) -> HttpResult {
    req.verify_user_permission(Permissions::UserRoleUnAssign)?;
    let db_pool = req.get_db_pool();
    let (_user_id, user_role_id) = path.into_inner();
    RoleService
        .un_assign_user_role(db_pool, user_role_id)
        .send_result()
}

#[get("{id}/individual-permissions")]
async fn individual_permissions(
    id: Path<Uuid>,
    q: Query<QueryParams>,
    req: HttpRequest,
) -> HttpResult {
    has_permission(req.to_owned(), Permissions::UserPermissionList, || {
        let db_pool = req.get_db_pool();
        UserPermissionRepository
            .list_paginated_by_user_id(db_pool, *id, q.into_inner())
            .send_pagination_result()
    })
}

#[get("{id}/assignable-permissions")]
async fn assignable_permissions(
    id: Path<Uuid>,
    req: HttpRequest,
    q: Query<QueryParams>,
) -> HttpResult {
    has_permission(req.to_owned(), Permissions::UserPermissionList, || {
        let db_pool = req.get_db_pool();
        UserPermissionRepository
            .list_assignable(db_pool, id.into_inner(), q.into_inner())
            .send_result()
    })
}

#[post("{id}/permissions")]
async fn add_permission(
    user_id: Path<Uuid>,
    form: Json<PermissionsParam>,
    req: HttpRequest,
) -> HttpResult {
    has_permission(req.to_owned(), Permissions::UserPermissionCreate, || {
        let db_pool = req.get_db_pool();
        let auth_user_id = req.auth_id();

        let mut permissions = vec![];
        let ids = form.into_inner().ids;
        for id in ids {
            let perm_result = RoleService.user_permission_add(db_pool, auth_user_id, *user_id, id);

            if let Ok(perm) = perm_result {
                permissions.push(perm);
            }
        }

        permissions.send_struct_result()
    })
}

#[delete("{id}/permissions/{pid}")]
async fn remove_permission(path: Path<(Uuid, Uuid)>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::UserPermissionDelete, || {
        let db_pool = req.get_db_pool();
        RoleService
            .user_permission_remove(db_pool, path.into_inner().1)
            .send_result()
    })
}

#[get("{id}/menus")]
async fn menus(id: Path<Uuid>, req: HttpRequest, q: Query<QueryParams>) -> HttpResult {
    has_permission(req.to_owned(), Permissions::UserUiMenuItemList, || {
        let db_pool = req.get_db_pool();
        UserUiMenuItemRepository
            .list_menu_by_user_id(db_pool, id.into_inner(), q.into_inner())
            .send_pagination_result()
    })
}

#[get("{id}/menu-items")]
async fn menu_items(id: Path<Uuid>, req: HttpRequest, q: Query<QueryParams>) -> HttpResult {
    has_permission(req.to_owned(), Permissions::UserUiMenuItemList, || {
        let db_pool = req.get_db_pool();
        UserUiMenuItemRepository
            .list_menu_item_by_user_id(db_pool, id.into_inner(), q.into_inner())
            .send_pagination_result()
    })
}

#[get("{id}/assignable-menu-items")]
async fn assignable_menu_items(id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::UserUiMenuItemList, || {
        let db_pool = req.get_db_pool();
        UserUiMenuItemRepository
            .list_assignable(db_pool, id.into_inner())
            .send_result()
    })
}

#[post("{id}/menu-items")]
async fn add_menu_item(
    user_id: Path<Uuid>,
    req: HttpRequest,
    form: Json<UserUiMenuItemCreateForm>,
) -> HttpResult {
    has_permission(req.to_owned(), Permissions::UserUiMenuItemList, || {
        let db_pool = req.get_db_pool();
        let auth_user_id = req.auth_id();

        let mut items: Vec<UserUiMenuItem> = vec![];
        for id in &form.ids {
            let res = UserUiMenuItemService.create(
                db_pool,
                auth_user_id,
                MenuItemCreateDto {
                    user_id: user_id.to_owned(),
                    menu_item_id: id.to_owned(),
                },
            );

            if let Ok(item) = res {
                items.push(item);
            } else {
                error!("{}", res.unwrap_err().to_string());
            }
        }

        Ok(items.send_response())
    })
}

#[delete("{id}/menu-items/{mid}")]
async fn remove_menu_item(path: Path<(Uuid, Uuid)>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::UserUiMenuItemDelete, || {
        let db_pool = req.get_db_pool();
        let ids = path.into_inner();
        UserUiMenuItemService
            .delete_by_item_id(db_pool, ids.0, ids.1)
            .map(|_| SuccessMessage("removed"))
            .send_result()
    })
}

#[post("{id}/passport")]
async fn upload_passport(
    req: HttpRequest,
    id: Path<Uuid>,
    form: MultipartForm<UploadForm>,
) -> HttpResult {
    let app = req.get_app_state();

    check_permission(req.to_owned(), Permissions::UserUploadPassport)?;

    let auth_user = UserRepository.find_by_id(app.get_db_pool(), *id)?;

    let file = FileUploadService.upload(
        app,
        form.into_inner().file,
        FileUploadData {
            uploader_id: auth_user.user_id,
            owner_id: *id,
            owner_type: Entities::User,
            description: Some(string("profile picture")),
            additional_info: None,
            is_temp: false,
        },
    )?;

    UserService
        .change_profile_picture(app.get_db_pool(), auth_user, file.file_path)
        .map(|u| u.into_sharable())
        .send_result()
}
