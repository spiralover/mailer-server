use crate::helpers::db_pagination::{PageData, Paginate};
use crate::helpers::get_db_conn;
use crate::helpers::http::QueryParams;
use crate::helpers::time::current_timestamp;
use crate::models::mail_error::MailError;
use crate::models::DBPool;
use crate::results::app_result::FormatAppResult;
use crate::results::AppResult;
use crate::schema::mail_errors;
use diesel::{ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl};
use std::ops::DerefMut;
use uuid::Uuid;

pub struct MailErrorRepository;

impl MailErrorRepository {
    pub fn list(
        &mut self,
        pool: &DBPool,
        query_params: QueryParams,
    ) -> AppResult<PageData<MailError>> {
        let search_format = format!("%{}%", query_params.get_search_query());
        mail_errors::table
            .filter(mail_errors::smtp_error.ilike(search_format.clone()))
            .order_by(mail_errors::created_at.desc())
            .paginate(query_params.get_page())
            .per_page(query_params.get_per_page())
            .load_and_count_pages::<MailError>(get_db_conn(pool).deref_mut())
            .into_app_result()
    }

    pub fn create(
        &mut self,
        pool: &DBPool,
        mail_id: Uuid,
        smtp_error: String,
    ) -> AppResult<MailError> {
        let model = MailError {
            mail_id,
            smtp_error,
            mail_error_id: Uuid::new_v4(),
            created_at: current_timestamp(),
        };

        diesel::insert_into(mail_errors::dsl::mail_errors)
            .values(model)
            .get_result::<MailError>(get_db_conn(pool).deref_mut())
            .into_app_result()
    }
}
