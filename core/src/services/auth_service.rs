use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tera::Context;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::enums::app_message::AppMessage;
use crate::helpers::DBPool;
use crate::helpers::id_generator::number_generator;
use crate::helpers::request::ClientInfo;
use crate::helpers::security::{AuthTokenData, generate_token};
use crate::helpers::string::password_verify;
use crate::models::auth_attempt::{AuthAttempt, AuthAttemptStatus, CreateDto};
use crate::models::mail::MailBox;
use crate::models::user::{
    FullName, LoginForm, User, UserRegisterForm, UserSharableData, UserStatus,
};
use crate::repositories::auth_attempt_repository::AuthAttemptRepository;
use crate::repositories::role_repository::RoleRepository;
use crate::repositories::user_repository::UserRepository;
use crate::results::AppResult;
use crate::results::http_result::ErroneousOptionResponse;
use crate::services::auth_attempt_service::AuthAttemptService;
use crate::services::mailer_service::MailerService;
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
    pub fn login(
        &mut self,
        app: Arc<AppState>,
        form: LoginForm,
        client: ClientInfo,
    ) -> AppResult<AppMessage> {
        let db_pool = app.database().clone();
        let user_lookup = UserRepository.find_by_email(&db_pool, form.email.to_owned());
        let context_less_error_message = Err(AppMessage::WarningMessage(
            "Invalid email address or password",
        ));

        let create_log = |user_id, code, error, status| {
            AuthAttemptService.create(
                &db_pool,
                status,
                CreateDto {
                    email: form.email.to_owned(),
                    ip_address: client.ip,
                    user_agent: client.ua,
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

            return Err(AppMessage::WarningMessage("Your account has not been verified, please check your mailbox or resend verification code"));
        }

        if user.has_started_password_reset.to_owned() {
            create_log(
                None,
                None,
                Some(String::from("user has started password reset")),
                AuthAttemptStatus::LoginDenied,
            )?;

            return Err(AppMessage::WarningMessage("You have started password reset, please follow the link sent to your email to complete it"));
        }

        if user.status == UserStatus::Pending.to_string() {
            create_log(
                None,
                None,
                Some(String::from("user is still in pending state")),
                AuthAttemptStatus::LoginDenied,
            )?;

            return Err(AppMessage::WarningMessage(
                "Your account has not been activated yet",
            ));
        }

        if user.status == UserStatus::Inactive.to_string() {
            create_log(
                None,
                None,
                Some(String::from("user is not activated")),
                AuthAttemptStatus::LoginDenied,
            )?;

            return Err(AppMessage::WarningMessage("Your account is not active"));
        }

        let code = number_generator(6);

        self.send_device_verification_code(user.clone(), code.clone(), app);

        create_log(
            Some(user.user_id),
            Some(code),
            None,
            AuthAttemptStatus::PendingVerification,
        )?;

        Ok(AppMessage::SuccessMessage(
            "Verification code mail sent, use it to verify your device",
        ))
    }

    pub fn send_device_verification_code(&mut self, user: User, code: String, app: Arc<AppState>) {
        let mut context = Context::new();
        context.insert("full_name", &user.full_name());
        context.insert("code", &code.to_owned());

        MailerService::new()
            .subject(app.title("Device Verification"))
            .receivers(vec![MailBox::new(&user.full_name(), user.email.as_str())])
            .view(app, "otp", context)
            .send_silently();
    }

    pub fn resend_device_verification_code(
        &mut self,
        app: Arc<AppState>,
        email: String,
    ) -> AppResult<AuthAttempt> {
        let db_pool = app.database();

        let user = UserRepository.find_by_email(db_pool, email.clone())?;
        let verification = AuthAttemptRepository.find_last_pending_by_email(db_pool, email)?;

        self.send_device_verification_code(
            user,
            verification.verification_code.clone().unwrap(),
            app,
        );

        Ok(verification)
    }

    pub fn verify_device(&mut self, pool: &DBPool, code: String) -> AppResult<AuthTokenData> {
        let verification = AuthAttemptService.verify_code(pool, code.clone())?;

        AuthAttemptService.change_code_status(pool, code, AuthAttemptStatus::LoggedIn)?;

        Ok(generate_token(
            verification.user_id.unwrap().to_string(),
            None,
            None,
        ))
    }

    pub fn get_profile(&mut self, pool: &DBPool, id: Uuid) -> AppResult<UserSharableData> {
        let perm_data = RoleService.list_user_permission_for_auth(pool, id.to_owned(), None)?;
        let items = UserUiMenuItemService.get_items_for_profile(pool, id)?;

        let mut user = perm_data.0.into_sharable();
        user.permissions = perm_data.1;
        user.menu_items = items;

        Ok(user)
    }

    pub fn register(&mut self, app: Arc<AppState>, form: UserRegisterForm) -> AppResult<User> {
        let db_pool = app.database();
        let existing = UserRepository.find_by_email(db_pool, form.email.clone());
        if existing.is_ok() {
            return Err(AppMessage::WarningMessage(
                "User with such email address already exists",
            ));
        }

        let default_role_id = RoleRepository.get_default_role_id(db_pool);
        UserService.create(app, default_role_id, form, Some(UserStatus::Pending))
    }

    pub fn logout(&mut self) {}
}
