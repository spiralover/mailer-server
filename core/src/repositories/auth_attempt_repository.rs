use std::ops::DerefMut;

use diesel::{
    BoolExpressionMethods, ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl,
};
use uuid::Uuid;

use crate::helpers::db::current_timestamp;
use crate::helpers::db_pagination::{Paginate, PaginationResult};
use crate::helpers::get_db_conn;
use crate::helpers::http::QueryParams;
use crate::models::auth_attempt::{AuthAttempt, AuthAttemptStatus, CreateDto};
use crate::models::DBPool;
use crate::results::app_result::FormatAppResult;
use crate::results::AppResult;
use crate::schema::auth_attempts;

pub struct AuthAttemptRepository;

impl AuthAttemptRepository {
    pub fn list_by_email(
        &mut self,
        pool: &DBPool,
        email: String,
        query_params: QueryParams,
    ) -> AppResult<PaginationResult<AuthAttempt>> {
        let search_format = format!("%{}%", query_params.get_search_query());
        auth_attempts::table
            .filter(
                auth_attempts::status
                    .ilike(search_format.clone())
                    .or(auth_attempts::email.ilike(search_format.clone()))
                    .or(auth_attempts::status.ilike(search_format.clone()))
                    .or(auth_attempts::user_agent.ilike(search_format.clone()))
                    .or(auth_attempts::ip_address.ilike(search_format)),
            )
            .filter(auth_attempts::email.eq(email))
            .filter(auth_attempts::deleted_at.is_null())
            .order_by(auth_attempts::created_at.desc())
            .paginate(query_params.get_page())
            .per_page(query_params.get_per_page())
            .load_and_count_pages::<AuthAttempt>(get_db_conn(pool).deref_mut())
            .into_app_result()
    }

    pub fn create(
        &mut self,
        pool: &DBPool,
        status: AuthAttemptStatus,
        data: CreateDto,
    ) -> AppResult<AuthAttempt> {
        let model = AuthAttempt {
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
        };

        diesel::insert_into(auth_attempts::dsl::auth_attempts)
            .values(model)
            .get_result::<AuthAttempt>(get_db_conn(pool).deref_mut())
            .into_app_result()
    }
}
