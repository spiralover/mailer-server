use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use nanoid::nanoid;
use std::sync::Arc;
use tera::Context;

use crate::app_state::AppState;
use crate::helpers::db::DatabaseConnectionHelper;
use crate::helpers::time::current_timestamp;
use crate::helpers::DBPool;
use crate::models::mail::MailBox;
use crate::models::password_reset::{PasswordReset, PasswordResetCreateDto, PasswordResetStatus};
use crate::models::user::{FullName, PasswordForm, User};
use crate::repositories::password_reset_repository::PasswordResetRepository;
use crate::repositories::user_repository::UserRepository;
use crate::results::app_result::FormatAppResult;
use crate::results::AppResult;
use crate::schema::password_resets;
use crate::services::mailer_service::MailerService;
use crate::services::user_service::UserService;

pub struct PasswordResetService;

impl PasswordResetService {
    pub fn send_link(
        &mut self,
        app: Arc<AppState>,
        dto: PasswordResetCreateDto,
    ) -> AppResult<PasswordReset> {
        let db_pool = app.database();
        let user = UserRepository.find_by_email(db_pool, dto.email.clone())?;

        let token = nanoid!();
        let reset = PasswordResetRepository.create(db_pool, user.user_id, token.clone(), dto)?;

        let _ = UserService.mark_user_started_password_reset(db_pool, user.clone());

        let mut context = Context::new();
        context.insert("full_name", &user.full_name());
        context.insert("token", &token);

        let subject = app.title("Password Reset");
        MailerService::new(app)
            .subject(subject)
            .receivers(vec![MailBox::new(&user.full_name(), user.email.as_str())])
            .view("password-reset", context)
            .send_silently();

        Ok(reset)
    }

    pub fn reset_password(
        &mut self,
        app: Arc<AppState>,
        token: String,
        password: PasswordForm,
    ) -> AppResult<User> {
        let db_pool = app.database();
        let reset = PasswordResetRepository.find_active_by_token(db_pool, token.clone())?;

        let user = UserService.change_password(db_pool, reset.user_id, password.password)?;
        let _ = self.mark_token_as_used(db_pool, token);
        let _ = UserService.mark_user_finished_password_reset(db_pool, user.clone());

        let mut context = Context::new();
        context.insert("full_name", &user.full_name());

        let subject = app.title("Changed Password");
        MailerService::new(app)
            .subject(subject)
            .receivers(vec![MailBox::new(&user.full_name(), user.email.as_str())])
            .view("password-reset-success", context)
            .send_silently();

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
        .execute(&mut pool.conn())
        .into_app_result()
    }
}
