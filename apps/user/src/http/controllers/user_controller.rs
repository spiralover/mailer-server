use actix_multipart::form::MultipartForm;
use actix_web::web::{block, Data, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, patch, post, put, HttpRequest};
use log::error;
use uuid::Uuid;
use validator::Validate;

use core::enums::app_message::AppMessage::SuccessMessage;
use core::enums::auth_permission::AuthPermission;
use core::enums::entities::Entities;
use core::helpers::form::IdsVecDto;
use core::helpers::http::{QueryParams, UploadForm};
use core::helpers::request::RequestHelper;
use core::helpers::string::string;
use core::helpers::DBPool;
use core::models::file_upload::FileUploadData;
use core::models::role::RoleParam;
use core::models::user::{PasswordForm, User, UserRegisterForm, UserStatus, UserUpdateForm};
use core::models::user_permission::PermissionsParam;
use core::models::user_ui_menu_item::{MenuItemCreateDto, UserUiMenuItem};
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
async fn index(q: Query<QueryParams>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::UserList)?;
        UserRepository
            .list(ctx.database(), q.0)
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
async fn store(form: Json<UserRegisterForm>, req: HttpRequest) -> HttpResult {
    form.validate()?;
    let ctx = req.context();

    block(move || {
        ctx.verify_user_permission(AuthPermission::UserCreate)?;
        let default_role_id = RoleRepository.get_default_role_id(ctx.database());
        UserService
            .create(ctx.app(), default_role_id, form.0, Some(UserStatus::Active))
            .map(|u| u.into_sharable())
    })
    .await
    .respond()
}

#[get("{id}")]
async fn show(id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::UserRead)?;
        UserService.get_profile(ctx.database(), *id)
    })
    .await
    .respond()
}

#[put("{id}")]
async fn update(form: Json<UserUpdateForm>, id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::UserUpdate)?;
        UserService.update(ctx.database(), *id, form.0)?;
        UserService.get_profile(ctx.database(), *id)
    })
    .await
    .respond()
}

#[patch("{id}/activate")]
async fn activate(id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::UserActivate)?;
        UserService
            .activate(ctx.database(), *id)
            .map(|u| u.into_sharable())
    })
    .await
    .respond()
}

#[patch("{id}/deactivate")]
async fn deactivate(id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::UserDeactivate)?;
        UserService
            .deactivate(ctx.database(), *id)
            .map(|u| u.into_sharable())
    })
    .await
    .respond()
}

#[patch("{id}/change-password")]
async fn change_password(id: Path<Uuid>, req: HttpRequest, form: Json<PasswordForm>) -> HttpResult {
    form.validate()?;
    let ctx = req.context();

    block(move || {
        ctx.verify_user_permission(AuthPermission::UserChangePassword)?;
        UserService
            .change_password(ctx.database(), *id, form.0.password)
            .map(|u| u.into_sharable())
    })
    .await
    .respond()
}

#[get("{id}/auth-attempts")]
async fn auth_attempts(id: Path<Uuid>, q: Query<QueryParams>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::UserAuthAttemptList)?;
        let email = UserRepository.fetch_email(ctx.database(), *id)?;
        AuthAttemptRepository.list_by_email(ctx.database(), email, q.0)
    })
    .await
    .respond()
}

#[get("{id}/roles")]
async fn roles(id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::UserRoleList)?;
        UserRoleRepository.list_paginated_by_user_id(ctx.database(), *id)
    })
    .await
    .respond()
}

#[post("{id}/roles")]
async fn assign_role(id: Path<Uuid>, form: Json<RoleParam>, req: HttpRequest) -> HttpResult {
    let role_id = form.0.role_id;
    let ctx = req.context();

    block(move || {
        ctx.verify_user_permission(AuthPermission::UserRoleAssign)?;
        RoleService.assign_role_to_user(ctx.database(), ctx.auth_id(), role_id, *id)
    })
    .await
    .respond()
}

#[get("{id}/assignable-roles")]
async fn assignable_roles(id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::UserRoleList)?;
        UserRoleRepository.list_assignable(ctx.database(), *id)
    })
    .await
    .respond()
}

#[delete("{id}/roles/{rid}")]
async fn un_assign_role(path: Path<(Uuid, Uuid)>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    let (_user_id, user_role_id) = path.into_inner();

    block(move || {
        ctx.verify_user_permission(AuthPermission::UserRoleUnAssign)?;
        RoleService.un_assign_user_role(ctx.database(), user_role_id)
    })
    .await
    .respond()
}

#[get("{id}/individual-permissions")]
async fn individual_permissions(
    id: Path<Uuid>,
    q: Query<QueryParams>,
    req: HttpRequest,
) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::UserPermissionList)?;
        UserPermissionRepository.list_paginated_by_user_id(ctx.database(), *id, q.0)
    })
    .await
    .respond()
}

#[get("{id}/assignable-permissions")]
async fn assignable_permissions(
    id: Path<Uuid>,
    req: HttpRequest,
    q: Query<QueryParams>,
) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::UserPermissionList)?;
        UserPermissionRepository.list_assignable(ctx.database(), *id, q.0)
    })
    .await
    .respond()
}

#[post("{id}/permissions")]
async fn add_permission(
    user_id: Path<Uuid>,
    form: Json<PermissionsParam>,
    req: HttpRequest,
) -> HttpResult {
    let ctx = req.context();

    block(move || {
        ctx.verify_user_permission(AuthPermission::UserPermissionCreate)?;

        let mut permissions = vec![];
        let ids = form.0.ids;
        for id in ids {
            let perm_result =
                RoleService.user_permission_add(ctx.database(), ctx.auth_id(), *user_id, id);

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
async fn remove_permission(path: Path<(Uuid, Uuid)>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::UserPermissionDelete)?;
        RoleService.user_permission_remove(ctx.database(), path.1)
    })
    .await
    .respond()
}

#[get("{id}/menus")]
async fn menus(id: Path<Uuid>, req: HttpRequest, q: Query<QueryParams>) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::UserUiMenuItemList)?;
        UserUiMenuItemRepository.list_menu_by_user_id(ctx.database(), *id, q.0)
    })
    .await
    .respond()
}

#[get("{id}/menu-items")]
async fn menu_items(id: Path<Uuid>, req: HttpRequest, q: Query<QueryParams>) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::UserUiMenuItemList)?;
        UserUiMenuItemRepository.list_menu_item_by_user_id(ctx.database(), *id, q.0)
    })
    .await
    .respond()
}

#[get("{id}/assignable-menu-items")]
async fn assignable_menu_items(id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::UserUiMenuItemList)?;
        UserUiMenuItemRepository.list_assignable(ctx.database(), *id)
    })
    .await
    .respond()
}

#[post("{id}/menu-items")]
async fn add_menu_item(
    user_id: Path<Uuid>,
    req: HttpRequest,
    pool: Data<DBPool>,
    form: Json<IdsVecDto>,
) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::UserUiMenuItemList)?;

        let mut items: Vec<UserUiMenuItem> = vec![];
        for id in &form.ids {
            let res = UserUiMenuItemService.create(
                pool.get_ref(),
                ctx.auth_id(),
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
async fn remove_menu_item(path: Path<(Uuid, Uuid)>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::UserUiMenuItemDelete)?;
        let ids = path.into_inner();
        UserUiMenuItemService
            .delete_by_item_id(ctx.database(), ids.0, ids.1)
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
) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::UserUploadPassport)?;
        let user = UserRepository.find_by_id(ctx.database(), *id)?;

        let file = FileUploadService.upload(
            ctx.app(),
            form.0.file,
            FileUploadData {
                uploader_id: ctx.auth_id(),
                owner_id: *id,
                owner_type: Entities::User,
                description: Some(string("profile picture")),
                additional_info: None,
                is_temp: false,
            },
        )?;

        UserService
            .change_profile_picture(ctx.database(), user, file.file_path)
            .map(|u| u.into_sharable())
    })
    .await
    .respond()
}
