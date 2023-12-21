use actix_web::web::Data;
use actix_web::HttpRequest;
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};

use crate::app_state::AppState;
use crate::enums::app_message::AppMessage::{UnAuthorized, WarningMessage};
use crate::enums::auth_permission::AuthPermission;
use crate::helpers::request::RequestHelper;
use crate::results::{AppResult, HttpResult};
use crate::services::auth_service::TokenClaims;
use crate::services::role_service::RoleService;

pub fn has_permission<F>(req: HttpRequest, p: AuthPermission, f: F) -> HttpResult
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

pub fn check_permission(req: HttpRequest, p: AuthPermission) -> AppResult<()> {
    let db_pool = req.app_data::<Data<AppState>>().unwrap().database();
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

pub(crate) fn decode_auth_token(
    raw: String,
    pat_prefix: String,
    app_key: String,
) -> jsonwebtoken::errors::Result<TokenData<TokenClaims>> {
    let token = raw.clone();
    let is_pat = token.starts_with(&pat_prefix.clone());
    let token = match is_pat {
        true => {
            let slices: Vec<&str> = token.split(&pat_prefix).collect();
            slices.get(1).unwrap().to_string()
        }
        false => token,
    };

    decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(app_key.as_ref()),
        &Validation::default(),
    )
}
