use crate::helpers::db::{DatabaseConnectionHelper, OptionalResult};
use crate::helpers::time::current_timestamp;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::helpers::DBPool;
use crate::models::password_reset::{PasswordReset, PasswordResetCreateDto, PasswordResetStatus};
use crate::results::app_result::FormatAppResult;
use crate::results::AppResult;
use crate::schema::password_resets;

pub struct PasswordResetRepository;

impl PasswordResetRepository {
    pub fn create(
        &mut self,
        pool: &DBPool,
        user_id: Uuid,
        token: String,
        dto: PasswordResetCreateDto,
    ) -> AppResult<PasswordReset> {
        diesel::insert_into(password_resets::dsl::password_resets)
            .values(PasswordReset {
                password_reset_id: Uuid::new_v4(),
                user_id,
                email: dto.email,
                token,
                ip_address: dto.ip_address,
                user_agent: dto.user_agent,
                status: PasswordResetStatus::AwaitingVerification.to_string(),
                created_at: current_timestamp(),
                updated_at: current_timestamp(),
            })
            .get_result::<PasswordReset>(&mut pool.conn())
            .into_app_result()
    }

    pub fn find_active_by_token(
        &mut self,
        pool: &DBPool,
        token: String,
    ) -> AppResult<PasswordReset> {
        password_resets::table
            .filter(password_resets::token.eq(token))
            .filter(
                password_resets::status.eq(PasswordResetStatus::AwaitingVerification.to_string()),
            )
            .first::<PasswordReset>(&mut pool.conn())
            .required("password reset link")
    }
}
