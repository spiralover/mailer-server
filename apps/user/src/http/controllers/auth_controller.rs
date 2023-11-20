use actix_web::http::StatusCode;
use actix_web::web::{Data, Json, Path, ServiceConfig};
use actix_web::{get, post, HttpRequest};
use validator::Validate;

use core::app_state::AppState;
use core::enums::app_message::AppMessage::{SuccessMessage, WarningMessage};
use core::helpers::http::get_ip_and_ua;
use core::helpers::request::RequestHelper;
use core::helpers::responder::json;
use core::http::middlewares::manual_auth_middleware::ManualAuthMiddleware;
use core::models::password_reset::PasswordResetCreateDto;
use core::models::user::{
    EmailForm, LoginForm, PasswordForm, UserRegisterForm, UsernameAvailability, UsernameForm,
};
use core::repositories::password_reset_repository::PasswordResetRepository;
use core::results::http_result::{ErroneousResponse, StructResponse};
use core::results::HttpResult;
use core::services::auth_service::AuthService;
use core::services::password_reset_service::PasswordResetService;
use core::services::user_service::UserService;

pub fn auth_controller(cfg: &mut ServiceConfig) {
    cfg.service(login);
    cfg.service(me);
    cfg.service(logout);
    cfg.service(profile);

    cfg.service(register);
    cfg.service(username_availability);

    cfg.service(resend_email_verification);
    cfg.service(send_password_reset_link);

    cfg.service(email_verification);
    cfg.service(reset_password);
    cfg.service(verify_password_reset_token);
}

#[post("login")]
async fn login(data: Json<LoginForm>, req: HttpRequest) -> HttpResult {
    AuthService
        .login(req, data.into_inner())
        .await
        .send_result()
}

#[get("me")]
async fn me(req: HttpRequest, _: ManualAuthMiddleware) -> HttpResult {
    let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
    let user = AuthService
        .get_profile(db_pool, req.auth_id())
        .map_err(|_| WarningMessage("Failed to get profile"))?;

    Ok(json(user, StatusCode::OK))
}

#[get("profile")]
async fn profile(req: HttpRequest, _: ManualAuthMiddleware) -> HttpResult {
    let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
    AuthService
        .get_profile(db_pool, req.auth_id())?
        .send_struct_result()
}

#[post("logout")]
async fn logout() -> HttpResult {
    AuthService.logout();
    SuccessMessage("Logged out successfully").ok()
}

#[post("username-availability")]
async fn username_availability(req: HttpRequest, form: Json<UsernameForm>) -> HttpResult {
    let db_pool = req.get_db_pool();
    let length = form.username.len();

    if length < 4 {
        return UsernameAvailability {
            is_available: false,
            username: Some(form.username.clone()),
            message: String::from("username must be at least 4 chars"),
        }
        .send_struct_result();
    }

    if length > 50 {
        return UsernameAvailability {
            is_available: false,
            username: Some(form.username.clone()),
            message: String::from("username must be less than 50 chars"),
        }
        .send_struct_result();
    }

    UserService
        .username_availability(db_pool, form.username.clone())
        .send_result()
}

#[post("register")]
async fn register(req: HttpRequest, form: Json<UserRegisterForm>) -> HttpResult {
    let app = req.app_data::<Data<AppState>>().unwrap().get_ref();
    form.validate()?;

    AuthService
        .register(app, form.into_inner())
        .await
        .map(|u| u.into_sharable())
        .send_result()
}

#[post("send-password-reset-link")]
async fn send_password_reset_link(req: HttpRequest, form: Json<EmailForm>) -> HttpResult {
    let app = req.app_data::<Data<AppState>>().unwrap().get_ref();
    let (ip_address, user_agent) = get_ip_and_ua(req.to_owned());

    PasswordResetService
        .send_link(
            app,
            PasswordResetCreateDto {
                ip_address,
                user_agent,
                email: form.into_inner().email,
            },
        )
        .await
        .map(|_| SuccessMessage("Password reset link has been sent to your email"))
        .send_result()
}

#[post("reset-password/{token}")]
async fn reset_password(
    req: HttpRequest,
    token: Path<String>,
    form: Json<PasswordForm>,
) -> HttpResult {
    let app = req.app_data::<Data<AppState>>().unwrap().get_ref();
    PasswordResetService
        .reset_password(app, token.into_inner(), form.into_inner())
        .await
        .map(|_| SuccessMessage("Your password has been changed"))
        .send_result()
}

#[get("verify-password-reset-token/{token}")]
async fn verify_password_reset_token(req: HttpRequest, token: Path<String>) -> HttpResult {
    let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
    PasswordResetRepository
        .find_active_by_token(db_pool, token.into_inner())
        .map(|_| SuccessMessage("link verified"))
        .send_result()
}

#[get("email-verification/{token}")]
async fn email_verification(req: HttpRequest, token: Path<String>) -> HttpResult {
    let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
    UserService
        .verify_email(db_pool, token.into_inner())
        .map(|_| SuccessMessage("account verified"))
        .send_result()
}

#[post("resend-email-verification")]
async fn resend_email_verification(req: HttpRequest, form: Json<EmailForm>) -> HttpResult {
    let app = req.app_data::<Data<AppState>>().unwrap().get_ref();
    UserService
        .resend_email_confirmation(app, form.into_inner().email)
        .await
        .map(|_| SuccessMessage("account verification code sent"))
        .send_result()
}
