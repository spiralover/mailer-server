use actix_multipart::form::MultipartForm;
use actix_web::web::{block, Data, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, patch, post, put, HttpRequest};
use log::error;
use uuid::Uuid;
use validator::Validate;

use core::app_state::AppState;
use core::enums::app_message::AppMessage::SuccessMessage;
use core::enums::auth_permission::AuthPermission;
use core::enums::entities::Entities;
use core::helpers::http::{QueryParams, UploadForm};
use core::helpers::request::RequestHelper;
use core::helpers::string::string;
use core::helpers::DBPool;
use core::models::file_upload::FileUploadData;
use core::models::role::RoleParam;
use core::models::user::{PasswordForm, User, UserRegisterForm, UserStatus, UserUpdateForm};
use core::models::user_permission::PermissionsParam;
use core::models::user_ui_menu_item::{
    MenuItemCreateDto, UserUiMenuItem, UserUiMenuItemCreateForm,
};
use core::repositories::auth_attempt_repository::AuthAttemptRepository;
use core::repositories::role_repository::RoleRepository;
use core::repositories::user_permission_repository::UserPermissionRepository;
use core::repositories::user_repository::UserRepository;
use core::repositories::user_role_repository::UserRoleRepository;
use core::repositories::user_ui_menu_item_repository::UserUiMenuItemRepository;
use core::results::http_result::ActixBlockingResultResponder;
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
async fn index(q: Query<QueryParams>, req: HttpRequest, pool: Data<DBPool>) -> HttpResult {
    req.verify_user_permission(AuthPermission::UserList)?;
    block(move || {
        UserRepository
            .list(pool.get_ref(), q.0)
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
    })
    .await
    .respond()
}

#[post("")]
async fn store(form: Json<UserRegisterForm>, req: HttpRequest, app: Data<AppState>) -> HttpResult {
    form.validate()?;
    req.verify_user_permission(AuthPermission::UserCreate)?;

    block(move || {
        let default_role_id = RoleRepository.get_default_role_id(app.database());
        UserService
            .create(
                app.into_inner(),
                default_role_id,
                form.0,
                Some(UserStatus::Active),
            )
            .map(|u| u.into_sharable())
    })
    .await
    .respond()
}

#[get("{id}")]
async fn show(id: Path<Uuid>, req: HttpRequest, pool: Data<DBPool>) -> HttpResult {
    req.verify_user_permission(AuthPermission::UserRead)?;
    block(move || UserService.get_profile(pool.get_ref(), *id))
        .await
        .respond()
}

#[put("{id}")]
async fn update(
    form: Json<UserUpdateForm>,
    id: Path<Uuid>,
    req: HttpRequest,
    pool: Data<DBPool>,
) -> HttpResult {
    req.verify_user_permission(AuthPermission::UserUpdate)?;
    block(move || {
        UserService.update(pool.get_ref(), *id, form.0)?;
        UserService.get_profile(pool.get_ref(), *id)
    })
    .await
    .respond()
}

#[patch("{id}/activate")]
async fn activate(id: Path<Uuid>, req: HttpRequest, pool: Data<DBPool>) -> HttpResult {
    req.verify_user_permission(AuthPermission::UserActivate)?;
    block(move || {
        UserService
            .activate(pool.get_ref(), *id)
            .map(|u| u.into_sharable())
    })
    .await
    .respond()
}

#[patch("{id}/deactivate")]
async fn deactivate(id: Path<Uuid>, req: HttpRequest, pool: Data<DBPool>) -> HttpResult {
    req.verify_user_permission(AuthPermission::UserDeactivate)?;
    block(move || {
        UserService
            .deactivate(pool.get_ref(), *id)
            .map(|u| u.into_sharable())
    })
    .await
    .respond()
}

#[patch("{id}/change-password")]
async fn change_password(
    id: Path<Uuid>,
    req: HttpRequest,
    pool: Data<DBPool>,
    form: Json<PasswordForm>,
) -> HttpResult {
    let form = form.0;
    form.validate()?;

    req.verify_user_permission(AuthPermission::UserChangePassword)?;

    block(move || {
        UserService
            .change_password(pool.get_ref(), *id, form.password)
            .map(|u| u.into_sharable())
    })
    .await
    .respond()
}

#[get("{id}/auth-attempts")]
async fn auth_attempts(
    id: Path<Uuid>,
    q: Query<QueryParams>,
    req: HttpRequest,
    pool: Data<DBPool>,
) -> HttpResult {
    req.verify_user_permission(AuthPermission::UserAuthAttemptList)?;
    block(move || {
        let email = UserRepository.fetch_email(pool.get_ref(), *id)?;
        AuthAttemptRepository.list_by_email(pool.get_ref(), email, q.0)
    })
    .await
    .respond()
}

#[get("{id}/roles")]
async fn roles(id: Path<Uuid>, req: HttpRequest, pool: Data<DBPool>) -> HttpResult {
    req.verify_user_permission(AuthPermission::UserRoleList)?;
    block(move || UserRoleRepository.list_paginated_by_user_id(pool.get_ref(), *id))
        .await
        .respond()
}

#[post("{id}/roles")]
async fn assign_role(
    id: Path<Uuid>,
    form: Json<RoleParam>,
    req: HttpRequest,
    pool: Data<DBPool>,
) -> HttpResult {
    req.verify_user_permission(AuthPermission::UserRoleAssign)?;

    let auth_id = req.auth_id();
    let role_id = form.0.role_id;

    block(move || RoleService.assign_role_to_user(pool.get_ref(), auth_id, role_id, *id))
        .await
        .respond()
}

#[get("{id}/assignable-roles")]
async fn assignable_roles(id: Path<Uuid>, req: HttpRequest, pool: Data<DBPool>) -> HttpResult {
    req.verify_user_permission(AuthPermission::UserRoleList)?;
    block(move || UserRoleRepository.list_assignable(pool.get_ref(), *id))
        .await
        .respond()
}

#[delete("{id}/roles/{rid}")]
async fn un_assign_role(
    path: Path<(Uuid, Uuid)>,
    req: HttpRequest,
    pool: Data<DBPool>,
) -> HttpResult {
    req.verify_user_permission(AuthPermission::UserRoleUnAssign)?;

    let (_user_id, user_role_id) = path.into_inner();

    block(move || RoleService.un_assign_user_role(pool.get_ref(), user_role_id))
        .await
        .respond()
}

#[get("{id}/individual-permissions")]
async fn individual_permissions(
    id: Path<Uuid>,
    q: Query<QueryParams>,
    req: HttpRequest,
    pool: Data<DBPool>,
) -> HttpResult {
    req.verify_user_permission(AuthPermission::UserPermissionList)?;
    block(move || UserPermissionRepository.list_paginated_by_user_id(pool.get_ref(), *id, q.0))
        .await
        .respond()
}

#[get("{id}/assignable-permissions")]
async fn assignable_permissions(
    id: Path<Uuid>,
    req: HttpRequest,
    q: Query<QueryParams>,
    pool: Data<DBPool>,
) -> HttpResult {
    req.verify_user_permission(AuthPermission::UserPermissionList)?;
    block(move || UserPermissionRepository.list_assignable(pool.get_ref(), *id, q.0))
        .await
        .respond()
}

#[post("{id}/permissions")]
async fn add_permission(
    user_id: Path<Uuid>,
    form: Json<PermissionsParam>,
    req: HttpRequest,
    pool: Data<DBPool>,
) -> HttpResult {
    req.verify_user_permission(AuthPermission::UserPermissionCreate)?;
    let auth_id = req.auth_id();

    block(move || {
        let mut permissions = vec![];
        let ids = form.0.ids;
        for id in ids {
            let perm_result =
                RoleService.user_permission_add(pool.get_ref(), auth_id, *user_id, id);

            if let Ok(perm) = perm_result {
                permissions.push(perm);
            }
        }

        Ok(permissions)
    })
    .await
    .respond()
}

#[delete("{id}/permissions/{pid}")]
async fn remove_permission(
    path: Path<(Uuid, Uuid)>,
    req: HttpRequest,
    pool: Data<DBPool>,
) -> HttpResult {
    req.verify_user_permission(AuthPermission::UserPermissionDelete)?;
    block(move || RoleService.user_permission_remove(pool.get_ref(), path.into_inner().1))
        .await
        .respond()
}

#[get("{id}/menus")]
async fn menus(
    id: Path<Uuid>,
    req: HttpRequest,
    pool: Data<DBPool>,
    q: Query<QueryParams>,
) -> HttpResult {
    req.verify_user_permission(AuthPermission::UserUiMenuItemList)?;
    block(move || UserUiMenuItemRepository.list_menu_by_user_id(pool.get_ref(), *id, q.0))
        .await
        .respond()
}

#[get("{id}/menu-items")]
async fn menu_items(
    id: Path<Uuid>,
    req: HttpRequest,
    pool: Data<DBPool>,
    q: Query<QueryParams>,
) -> HttpResult {
    req.verify_user_permission(AuthPermission::UserUiMenuItemList)?;
    block(move || UserUiMenuItemRepository.list_menu_item_by_user_id(pool.get_ref(), *id, q.0))
        .await
        .respond()
}

#[get("{id}/assignable-menu-items")]
async fn assignable_menu_items(id: Path<Uuid>, req: HttpRequest, pool: Data<DBPool>) -> HttpResult {
    req.verify_user_permission(AuthPermission::UserUiMenuItemList)?;
    block(move || UserUiMenuItemRepository.list_assignable(pool.get_ref(), *id))
        .await
        .respond()
}

#[post("{id}/menu-items")]
async fn add_menu_item(
    user_id: Path<Uuid>,
    req: HttpRequest,
    pool: Data<DBPool>,
    form: Json<UserUiMenuItemCreateForm>,
) -> HttpResult {
    req.verify_user_permission(AuthPermission::UserUiMenuItemList)?;
    let auth_id = req.auth_id();

    block(move || {
        let mut items: Vec<UserUiMenuItem> = vec![];
        for id in &form.ids {
            let res = UserUiMenuItemService.create(
                pool.get_ref(),
                auth_id,
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

        Ok(items)
    })
    .await
    .respond()
}

#[delete("{id}/menu-items/{mid}")]
async fn remove_menu_item(
    path: Path<(Uuid, Uuid)>,
    req: HttpRequest,
    pool: Data<DBPool>,
) -> HttpResult {
    req.verify_user_permission(AuthPermission::UserUiMenuItemDelete)?;
    block(move || {
        let ids = path.into_inner();
        UserUiMenuItemService
            .delete_by_item_id(pool.get_ref(), ids.0, ids.1)
            .map(|_| SuccessMessage("removed"))
    })
    .await
    .respond()
}

#[post("{id}/passport")]
async fn upload_passport(
    req: HttpRequest,
    id: Path<Uuid>,
    form: MultipartForm<UploadForm>,
    app: Data<AppState>,
) -> HttpResult {
    req.verify_user_permission(AuthPermission::UserUploadPassport)?;
    let auth_id = req.auth_id();

    block(move || {
        let pool = app.database().clone();
        let user = UserRepository.find_by_id(&pool, *id)?;

        let file = FileUploadService.upload(
            app.clone().into_inner(),
            form.0.file,
            FileUploadData {
                uploader_id: auth_id,
                owner_id: *id,
                owner_type: Entities::User,
                description: Some(string("profile picture")),
                additional_info: None,
                is_temp: false,
            },
        )?;

        UserService
            .change_profile_picture(&pool, user, file.file_path)
            .map(|u| u.into_sharable())
    })
    .await
    .respond()
}
