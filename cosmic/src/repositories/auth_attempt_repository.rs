use diesel::{
    BoolExpressionMethods, ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl,
};
use uuid::Uuid;

use crate::helpers::db::{DatabaseConnectionHelper, OptionalResult};
use crate::helpers::db_pagination::Paginate;
use crate::helpers::http::QueryParams;
use crate::helpers::time::current_timestamp;
use crate::models::auth_attempt::{AuthAttempt, AuthAttemptStatus, CreateDto};
use crate::helpers::DBPool;
use crate::results::{AppPaginationResult, AppResult};
use crate::results::app_result::FormatAppResult;
use crate::schema::auth_attempts;

pub struct AuthAttemptRepository;

impl AuthAttemptRepository {
    pub fn list_by_email(
        &mut self,
        pool: &DBPool,
        email: String,
        q: QueryParams,
    ) -> AppPaginationResult<AuthAttempt> {
        auth_attempts::table
            .filter(
                auth_attempts::status
                    .ilike(q.get_search_query_like())
                    .or(auth_attempts::email.ilike(q.get_search_query_like()))
                    .or(auth_attempts::status.ilike(q.get_search_query_like()))
                    .or(auth_attempts::user_agent.ilike(q.get_search_query_like()))
                    .or(auth_attempts::ip_address.ilike(q.get_search_query_like())),
            )
            .filter(auth_attempts::email.eq(email))
            .filter(auth_attempts::deleted_at.is_null())
            .order_by(auth_attempts::created_at.desc())
            .paginate(q.get_page())
            .per_page(q.get_per_page())
            .load_and_count_pages::<AuthAttempt>(&mut pool.conn())
            .into_app_result()
    }

    pub fn create(
        &mut self,
        pool: &DBPool,
        status: AuthAttemptStatus,
        data: CreateDto,
    ) -> AppResult<AuthAttempt> {
        diesel::insert_into(auth_attempts::dsl::auth_attempts)
            .values(AuthAttempt {
                auth_attempt_id: Uuid::new_v4(),
                user_id: data.user_id,
                email: data.email,
                ip_address: data.ip_address,
                user_agent: data.user_agent,
                auth_error: None,
                verification_code: data.verification_code,
                verification_code_trials: 0,
                status: status.to_string(),
                created_at: current_timestamp(),
                updated_at: current_timestamp(),
                deleted_at: None,
            })
            .get_result::<AuthAttempt>(&mut pool.conn())
            .into_app_result()
    }

    pub fn find_by_code(&mut self, pool: &DBPool, code: String) -> AppResult<AuthAttempt> {
        auth_attempts::table
            .filter(auth_attempts::verification_code.eq(code))
            .filter(auth_attempts::deleted_at.is_null())
            .first::<AuthAttempt>(&mut pool.conn())
            .required("authentication attempt")
    }

    pub fn find_last_pending_by_email(
        &mut self,
        pool: &DBPool,
        email: String,
    ) -> AppResult<AuthAttempt> {
        auth_attempts::table
            .filter(auth_attempts::email.eq(email))
            .filter(auth_attempts::deleted_at.is_null())
            .filter(auth_attempts::status.eq(AuthAttemptStatus::PendingVerification.to_string()))
            .order_by(auth_attempts::created_at.desc())
            .limit(1)
            .first::<AuthAttempt>(&mut pool.conn())
            .required("authentication attempt")
    }

    pub fn find_pending_verification_by_code(
        &mut self,
        pool: &DBPool,
        code: String,
    ) -> AppResult<AuthAttempt> {
        auth_attempts::table
            .filter(auth_attempts::verification_code.eq(code))
            .filter(auth_attempts::deleted_at.is_null())
            .filter(auth_attempts::status.eq(AuthAttemptStatus::PendingVerification.to_string()))
            .first::<AuthAttempt>(&mut pool.conn())
            .required("authentication attempt")
    }

    pub fn deactivate_all_active_for_user(
        &mut self,
        pool: &DBPool,
        email: String,
    ) -> AppResult<usize> {
        diesel::update(
            auth_attempts::dsl::auth_attempts
                .filter(auth_attempts::email.eq(email))
                .filter(
                    auth_attempts::status.eq(AuthAttemptStatus::PendingVerification.to_string()),
                ),
        )
            .set((
                auth_attempts::status.eq(AuthAttemptStatus::InvalidatedToken.to_string()),
                auth_attempts::updated_at.eq(current_timestamp()),
            ))
            .execute(&mut pool.conn())
            .into_app_result()
    }
}
