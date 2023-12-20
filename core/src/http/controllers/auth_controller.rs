use actix_web::{get, HttpRequest, post};
use actix_web::web::{block, Data, Json, Path, ServiceConfig};

use crate::app_state::AppState;
use crate::enums::app_message::AppMessage::{SuccessMessage, WarningMessage};
use crate::helpers::DBPool;
use crate::helpers::request::RequestHelper;
use crate::http::middlewares::manual_auth_middleware::ManualAuthMiddleware;
use crate::models::auth_attempt::LoginToken;
use crate::models::password_reset::PasswordResetCreateDto;
use crate::models::user::{
    EmailForm, LoginForm, PasswordForm, UserRegisterForm,
};
use crate::repositories::password_reset_repository::PasswordResetRepository;
use crate::results::http_result::ActixBlockingResultResponder;
use crate::results::HttpResult;
use crate::services::auth_service::AuthService;
use crate::services::password_reset_service::PasswordResetService;
use crate::services::user_service::UserService;

pub fn auth_controller(cfg: &mut ServiceConfig) {
    cfg.service(login);
    cfg.service(me);
    cfg.service(logout);
    cfg.service(profile);

    cfg.service(register);
    cfg.service(verify_device);

    cfg.service(resend_verification_code);
    cfg.service(resend_email_verification);
    cfg.service(send_password_reset_link);

    cfg.service(email_verification);
    cfg.service(reset_password);
    cfg.service(verify_password_reset_token);
}

#[post("login")]
async fn login(data: Json<LoginForm>, app: Data<AppState>, req: HttpRequest) -> HttpResult {
    let client_info = req.get_client_info();
    block(move || AuthService.login(app.into_inner(), data.into_inner(), client_info))
        .await
        .respond()
}

#[post("resend-device-verification-code")]
async fn resend_verification_code(data: Json<EmailForm>, app: Data<AppState>) -> HttpResult {
    block(move || {
        AuthService
            .resend_device_verification_code(app.into_inner(), data.into_inner().email)
            .map(|_| SuccessMessage("Device verification code has been resent"))
    })
        .await
        .respond()
}

#[post("verify-device")]
async fn verify_device(pool: Data<DBPool>, data: Json<LoginToken>) -> HttpResult {
    block(move || {
        AuthService
            .verify_device(pool.get_ref(), data.into_inner().code)
            .map_err(|_| WarningMessage("Invalid device verification code"))
    })
        .await
        .respond()
}

#[get("me")]
async fn me(req: HttpRequest, pool: Data<DBPool>, _: ManualAuthMiddleware) -> HttpResult {
    let auth_id = req.auth_id();
    block(move || {
        AuthService
            .get_profile(pool.get_ref(), auth_id)
            .map_err(|_| WarningMessage("Failed to get profile"))
    })
        .await
        .respond()
}

#[get("profile")]
async fn profile(req: HttpRequest, pool: Data<DBPool>, _: ManualAuthMiddleware) -> HttpResult {
    let auth_id = req.auth_id();
    block(move || AuthService.get_profile(pool.get_ref(), auth_id))
        .await
        .respond()
}

#[post("logout")]
async fn logout() -> HttpResult {
    AuthService.logout();
    SuccessMessage("Logged out successfully").ok()
}

#[post("register")]
async fn register(app: Data<AppState>, form: Json<UserRegisterForm>) -> HttpResult {
    block(move || AuthService.register(app.into_inner(), form.into_inner()))
        .await
        .respond()
}

#[post("send-password-reset-link")]
async fn send_password_reset_link(
    req: HttpRequest,
    app: Data<AppState>,
    form: Json<EmailForm>,
) -> HttpResult {
    let client_info = req.get_client_info();
    block(move || {
        PasswordResetService
            .send_link(
                app.into_inner(),
                PasswordResetCreateDto {
                    ip_address: client_info.ip,
                    user_agent: client_info.ua,
                    email: form.into_inner().email,
                },
            )
            .map(|_| SuccessMessage("Password reset link has been sent to your email"))
    })
        .await
        .respond()
}

#[post("reset-password/{token}")]
async fn reset_password(
    app: Data<AppState>,
    token: Path<String>,
    form: Json<PasswordForm>,
) -> HttpResult {
    block(move || {
        PasswordResetService
            .reset_password(app.into_inner(), token.into_inner(), form.into_inner())
            .map(|_| SuccessMessage("Your password has been changed"))
    })
        .await
        .respond()
}

#[get("verify-password-reset-token/{token}")]
async fn verify_password_reset_token(pool: Data<DBPool>, token: Path<String>) -> HttpResult {
    block(move || {
        PasswordResetRepository
            .find_active_by_token(pool.get_ref(), token.into_inner())
            .map(|_| SuccessMessage("link verified"))
    })
        .await
        .respond()
}

#[get("email-verification/{token}")]
async fn email_verification(pool: Data<DBPool>, token: Path<String>) -> HttpResult {
    block(move || {
        UserService
            .verify_email(pool.get_ref(), token.into_inner())
            .map(|_| SuccessMessage("account verified"))
    })
        .await
        .respond()
}

#[post("resend-email-verification")]
async fn resend_email_verification(app: Data<AppState>, form: Json<EmailForm>) -> HttpResult {
    block(move || {
        UserService
            .resend_email_confirmation(app.into_inner(), form.into_inner().email)
            .map(|_| SuccessMessage("account verification code sent"))
    })
        .await
        .respond()
}
