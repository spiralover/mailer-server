use std::ops::DerefMut;

use diesel::{
    BoolExpressionMethods, ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl,
};
use uuid::Uuid;

use crate::helpers::db::{current_timestamp, OptionalResult};
use crate::helpers::db_pagination::{Paginate, PaginationResult};
use crate::helpers::get_db_conn;
use crate::helpers::http::QueryParams;
use crate::models::application::{
    Application, ApplicationCreateForm, ApplicationStatus, ApplicationUpdateForm,
};
use crate::models::DBPool;
use crate::results::app_result::FormatAppResult;
use crate::results::http_result::ErroneousOptionResponse;
use crate::results::AppResult;
use crate::schema::applications;

pub struct ApplicationRepository;

impl ApplicationRepository {
    pub fn list(
        &mut self,
        pool: &DBPool,
        query_params: QueryParams,
    ) -> AppResult<PaginationResult<Application>> {
        let search_format = format!("%{}%", query_params.get_search_query());
        applications::table
            .filter(
                applications::name
                    .ilike(search_format.clone())
                    .or(applications::url.ilike(search_format.clone()))
                    .or(applications::description.ilike(search_format)),
            )
            .filter(applications::deleted_at.is_null())
            .order_by(applications::created_at.desc())
            .paginate(query_params.get_page())
            .per_page(query_params.get_per_page())
            .load_and_count_pages::<Application>(get_db_conn(pool).deref_mut())
            .into_app_result()
    }

    pub fn create(
        &mut self,
        pool: &DBPool,
        created_by: Uuid,
        data: ApplicationCreateForm,
    ) -> Application {
        let model = Application {
            application_id: Uuid::new_v4(),
            created_by,
            name: data.name,
            code: data.code.unwrap(),
            url: data.url,
            webhook: data.webhook,
            description: data.description,
            logo: String::from(""),
            status: application_stringy_status(ApplicationStatus::Active)
                .parse()
                .unwrap(),
            created_at: current_timestamp(),
            updated_at: current_timestamp(),
            deleted_at: None,
        };

        diesel::insert_into(applications::dsl::applications)
            .values(model)
            .get_result::<Application>(get_db_conn(pool).deref_mut())
            .unwrap()
    }

    pub fn update(
        &mut self,
        pool: &DBPool,
        id: Uuid,
        form: ApplicationUpdateForm,
    ) -> AppResult<Application> {
        let result = self.find_by_id(pool, id);

        if result.is_error_or_empty() {
            return result.get_error_result();
        }

        diesel::update(applications::dsl::applications.filter(applications::application_id.eq(id)))
            .set((
                applications::name.eq(form.name),
                applications::url.eq(form.url),
                applications::webhook.eq(form.webhook),
                applications::description.eq(form.description),
                applications::updated_at.eq(current_timestamp()),
            ))
            .execute(get_db_conn(pool).deref_mut())
            .expect("Failed to update application");

        Ok(self.find_by_id(pool, id).unwrap())
    }

    pub fn delete(&mut self, pool: &DBPool, id: Uuid) -> AppResult<Application> {
        let result = self.find_by_id(pool, id);

        if result.is_error_or_empty() {
            return result.get_error_result();
        }

        diesel::update(applications::dsl::applications.filter(applications::application_id.eq(id)))
            .set((applications::deleted_at.eq(current_timestamp()),))
            .execute(get_db_conn(pool).deref_mut())
            .into_app_result()?;

        Ok(self.find_by_id(pool, id).unwrap())
    }

    pub fn find_by_id(&mut self, pool: &DBPool, id: Uuid) -> AppResult<Application> {
        applications::table
            .filter(applications::application_id.eq(id))
            .filter(applications::deleted_at.is_null())
            .first::<Application>(get_db_conn(pool).deref_mut())
            .required("application")
    }

    pub fn find_owned_by_id(&mut self, pool: &DBPool, id: Uuid, owner: Uuid) -> AppResult<Uuid> {
        applications::table
            .select(applications::application_id)
            .filter(applications::application_id.eq(id))
            .filter(applications::created_by.eq(owner))
            .filter(applications::deleted_at.is_null())
            .first::<Uuid>(get_db_conn(pool).deref_mut())
            .required("application")
    }

    pub fn find_by_code(&mut self, pool: &DBPool, code: String) -> AppResult<Application> {
        applications::table
            .filter(applications::code.eq(code))
            .filter(applications::deleted_at.is_null())
            .first::<Application>(get_db_conn(pool).deref_mut())
            .required("application")
    }
}

pub fn application_stringy_status(status: ApplicationStatus) -> &'static str {
    match status {
        ApplicationStatus::Active => "active",
        ApplicationStatus::Inactive => "inactive",
    }
}
