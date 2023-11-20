use actix_web::web::Data;
use actix_web::HttpRequest;

use crate::app_state::AppState;
use crate::enums::app_message::AppMessage::{UnAuthorized, WarningMessage};
use crate::helpers::request::RequestHelper;
use crate::permissions::Permissions;
use crate::results::{AppResult, HttpResult};
use crate::services::role_service::RoleService;

pub fn has_permission<F>(req: HttpRequest, p: Permissions, f: F) -> HttpResult
where
    F: FnOnce() -> HttpResult,
{
    let checked_permission = check_permission(req, p);
    let has_permission = checked_permission.is_ok();

    if has_permission {
        return f();
    }

    Err(UnAuthorized)
}

pub fn check_permission(req: HttpRequest, p: Permissions) -> AppResult<()> {
    let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
    let user_id = req.auth_id();
    let perm_result =
        RoleService.list_user_permission_for_auth(db_pool, user_id, Some(p.to_string()));
    if perm_result.is_err() {
        return Err(WarningMessage("Failed to acquire user permissions"));
    }

    // let perm = UserPermissionRepository.find(pool, p.to_string(), user_id);
    // return perm.send_result();

    let permissions = perm_result.unwrap().1;
    if permissions.is_empty() {
        return Err(UnAuthorized);
    }

    Ok(())
}
