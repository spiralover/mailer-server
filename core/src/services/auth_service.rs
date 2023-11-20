use actix_web::web::Data;
use actix_web::HttpRequest;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app_state::AppState;
use crate::enums::app_message::AppMessage::WarningMessage;
use crate::helpers::http::get_ip_and_ua;
use crate::helpers::id_generator::number_generator;
use crate::helpers::security::{generate_token, AuthTokenData};
use crate::helpers::string::password_verify;
use crate::models::auth_attempt::{AuthAttemptStatus, CreateDto};
use crate::models::user::{LoginForm, User, UserRegisterForm, UserSharableData, UserStatus};
use crate::models::DBPool;
use crate::repositories::role_repository::RoleRepository;
use crate::repositories::user_repository::UserRepository;
use crate::results::http_result::ErroneousOptionResponse;
use crate::results::AppResult;
use crate::services::auth_attempt_service::AuthAttemptService;
use crate::services::role_service::RoleService;
use crate::services::user_service::UserService;
use crate::services::user_ui_menu_item_service::UserUiMenuItemService;

pub struct AuthService;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

impl AuthService {
    pub async fn login(&mut self, req: HttpRequest, form: LoginForm) -> AppResult<AuthTokenData> {
        let app = req.app_data::<Data<AppState>>().unwrap().get_ref();
        let db_pool = app.get_db_pool();
        let user_lookup = UserRepository.find_by_email(db_pool, form.email.to_owned());
        let context_less_error_message = Err(WarningMessage("Invalid email address or password"));

        let (ip_address, user_agent) = get_ip_and_ua(req.to_owned());

        let create_log = |user_id, code, error, status| {
            AuthAttemptService.create(
                db_pool,
                status,
                CreateDto {
                    email: form.email.to_owned(),
                    ip_address,
                    user_agent,
                    user_id,
                    verification_code: code,
                    auth_error: error,
                },
            )
        };

        if user_lookup.is_error_or_empty() {
            create_log(
                None,
                None,
                Some(String::from("user does not exists")),
                AuthAttemptStatus::InvalidCredential,
            )?;

            return context_less_error_message;
        }

        let user = user_lookup.unwrap();

        if !password_verify(user.password.as_str(), form.password.as_str()) {
            create_log(
                None,
                None,
                Some(String::from("user does not exists")),
                AuthAttemptStatus::InvalidCredential,
            )?;

            return context_less_error_message;
        }

        if !user.is_verified.to_owned() {
            create_log(
                None,
                None,
                Some(String::from("user is not verified")),
                AuthAttemptStatus::LoginDenied,
            )?;

            return Err(WarningMessage("Your account has not been verified, please check your mailbox or resend verification code"));
        }

        if user.has_started_password_reset.to_owned() {
            create_log(
                None,
                None,
                Some(String::from("user has started password reset")),
                AuthAttemptStatus::LoginDenied,
            )?;

            return Err(WarningMessage("You have started password reset, please follow the link sent to your email to complete it"));
        }

        if user.status == UserStatus::Pending.to_string() {
            create_log(
                None,
                None,
                Some(String::from("user is still in pending state")),
                AuthAttemptStatus::LoginDenied,
            )?;

            return Err(WarningMessage("Your account has not been activated yet"));
        }

        if user.status == UserStatus::Inactive.to_string() {
            create_log(
                None,
                None,
                Some(String::from("user is not activated")),
                AuthAttemptStatus::LoginDenied,
            )?;

            return Err(WarningMessage("Your account is not active"));
        }

        let code = number_generator(6);

        create_log(
            Some(user.user_id),
            Some(code),
            None,
            AuthAttemptStatus::LoggedIn,
        )?;

        Ok(generate_token(user.user_id.to_string(), None))
    }

    pub fn get_profile(&mut self, pool: &DBPool, id: Uuid) -> AppResult<UserSharableData> {
        let perm_data = RoleService.list_user_permission_for_auth(pool, id.to_owned(), None)?;
        let items = UserUiMenuItemService.get_items_for_profile(pool, id)?;

        let mut user = perm_data.0.into_sharable();
        user.permissions = perm_data.1;
        user.menu_items = items;

        Ok(user)
    }

    pub async fn register(&mut self, app: &AppState, form: UserRegisterForm) -> AppResult<User> {
        let db_pool = app.get_db_pool();
        let existing = UserRepository.find_by_email(db_pool, form.email.clone());
        if existing.is_ok() {
            return Err(WarningMessage(
                "User with such email address already exists",
            ));
        }

        let default_role_id = RoleRepository.get_default_role_id(db_pool);
        UserService
            .create(app, default_role_id, form, Some(UserStatus::Pending))
            .await
    }

    pub fn logout(&mut self) {}
}
