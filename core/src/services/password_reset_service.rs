use std::ops::DerefMut;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use nanoid::nanoid;
use tera::Context;

use crate::app_state::AppState;
use crate::helpers::db::current_timestamp;
use crate::helpers::get_db_conn;
use crate::models::mail::MailBox;
use crate::models::password_reset::{PasswordReset, PasswordResetCreateDto, PasswordResetStatus};
use crate::models::user::{FullName, PasswordForm, User};
use crate::models::DBPool;
use crate::repositories::password_reset_repository::PasswordResetRepository;
use crate::repositories::user_repository::UserRepository;
use crate::results::app_result::FormatAppResult;
use crate::results::http_result::ErroneousOptionResponse;
use crate::results::AppResult;
use crate::schema::password_resets;
use crate::services::mailer_service::MailerService;
use crate::services::user_service::UserService;

pub struct PasswordResetService;

impl PasswordResetService {
    pub async fn send_link(
        &mut self,
        app: &AppState,
        dto: PasswordResetCreateDto,
    ) -> AppResult<PasswordReset> {
        let db_pool = app.get_db_pool();
        let user_result = UserRepository.find_by_email(db_pool, dto.email.clone());
        if user_result.is_error_or_empty() {
            return Err(user_result.unwrap_err());
        }

        let token = nanoid!();
        let user = user_result.unwrap();
        let reset = PasswordResetRepository.create(db_pool, user.user_id, token.clone(), dto)?;

        let _ = UserService.mark_user_started_password_reset(db_pool, user.clone());

        let mut context = Context::new();
        context.insert("full_name", &user.full_name());
        context.insert("token", &token);

        MailerService::new()
            .subject(app.title("Password Reset"))
            .receivers(vec![MailBox::new(&user.full_name(), user.email.as_str())])
            .view(app, "password-reset", context)
            .send_silently()
            .await;

        Ok(reset)
    }

    pub async fn reset_password(
        &mut self,
        app: &AppState,
        token: String,
        password: PasswordForm,
    ) -> AppResult<User> {
        let db_pool = app.get_db_pool();
        let reset = PasswordResetRepository.find_active_by_token(db_pool, token.clone())?;

        let user = UserService.change_password(db_pool, reset.user_id, password.password)?;
        let _ = self.mark_token_as_used(db_pool, token);
        let _ = UserService.mark_user_finished_password_reset(db_pool, user.clone());

        let mut context = Context::new();
        context.insert("full_name", &user.full_name());

        MailerService::new()
            .subject(app.title("Changed Password"))
            .receivers(vec![MailBox::new(&user.full_name(), user.email.as_str())])
            .view(app, "password-reset-success", context)
            .send_silently()
            .await;

        Ok(user)
    }

    fn mark_token_as_used(&mut self, pool: &DBPool, token: String) -> AppResult<usize> {
        diesel::update(
            password_resets::dsl::password_resets.filter(password_resets::token.eq(token)),
        )
        .set((
            password_resets::status.eq(PasswordResetStatus::Completed.to_string()),
            password_resets::updated_at.eq(current_timestamp()),
        ))
        .execute(get_db_conn(pool).deref_mut())
        .into_app_result()
    }
}
