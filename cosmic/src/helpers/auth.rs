use std::str::FromStr;

use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::HttpRequest;
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use log::{debug, error};
use uuid::Uuid;

use crate::app_state::AppState;
use crate::enums::app_message::AppMessage;
use crate::enums::app_message::AppMessage::{UnAuthorized, WarningMessageStr};
use crate::enums::auth_permission::AuthPermission;
use crate::helpers::once_lock::OnceLockHelper;
use crate::helpers::request::RequestHelper;
use crate::helpers::responder::{JsonResponse, JsonResponseEmptyMessage};
use crate::helpers::DBPool;
use crate::models::user::UserCacheData;
use crate::repositories::personal_access_token_repository::PersonaAccessTokenRepository;
use crate::repositories::user_repository::UserRepository;
use crate::repositories::user_role_repository::UserRoleRepository;
use crate::results::{AppResult, HttpResult};
use crate::services::auth_service::TokenClaims;
use crate::services::role_service::RoleService;
use crate::services::user_service::UserService;
use crate::MAILER;

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
        return Err(WarningMessageStr("Failed to acquire user permissions"));
    }

    // let perm = UserPermissionRepository.find(pool, p.to_string(), user_id);
    // return perm.send_result();

    let permissions = perm_result.unwrap().1;
    if permissions.is_empty() {
        return Err(UnAuthorized);
    }

    Ok(())
}

pub fn verify_auth_permission(pool: &DBPool, auth_id: Uuid, p: AuthPermission) -> AppResult<()> {
    let perm_result = RoleService.list_user_permission_for_auth(pool, auth_id, Some(p.to_string()));
    if perm_result.is_err() {
        return Err(WarningMessageStr("Failed to acquire user permissions"));
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

pub(crate) fn make_unauthorized_message(msg: &str) -> JsonResponse<JsonResponseEmptyMessage> {
    JsonResponse {
        code: 401,
        success: false,
        status: StatusCode::UNAUTHORIZED.to_string(),
        message: Some(msg.to_string()),
        data: JsonResponseEmptyMessage {},
    }
}

pub(crate) fn get_auth_user(str_uuid: String) -> AppResult<UserCacheData> {
    MAILER
        .cache()
        .get_or_put::<UserCacheData, _>(&str_uuid, |_| {
            debug!("caching user({})...", str_uuid);

            let user_id = Uuid::from_str(&str_uuid).unwrap();
            let user = UserRepository.fetch_cacheable(MAILER.database(), user_id)?;

            UserService::make_cache_date(user)
        })
}

pub(crate) fn fetch_pat_user(token: String) -> AppResult<UserCacheData> {
    MAILER.cache().get_or_put::<UserCacheData, _>(&token, |c| {
        let pat_result =
            PersonaAccessTokenRepository.find_by_token(MAILER.database(), token.clone());

        match pat_result {
            Ok(pat) => {
                if !pat.is_usable() {
                    let msg = Box::leak(Box::new(format!(
                        "personal access token has expired on {:?}",
                        pat.expired_at
                    )));

                    let _ = c.delete(&token);

                    return Err(AppMessage::UnAuthorizedMessage(msg));
                }

                let mut user = UserRepository
                    .fetch_cacheable(MAILER.database(), pat.user_id)
                    .map(|user| user.into_cache_data())?;

                let roles = UserRoleRepository
                    .list_role_names_by_user_id(MAILER.database(), user.user_id)?;
                user.roles = roles;

                Ok(user)
            }
            Err(err) => {
                error!("pat error: {:?}", err);
                Err(AppMessage::UnAuthorizedMessage(
                    "failed to authenticate personal access token",
                ))
            }
        }
    })
}
