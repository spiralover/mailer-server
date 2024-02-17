use diesel::{
    BoolExpressionMethods, ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl,
    SaveChangesDsl,
};
use uuid::Uuid;

use crate::helpers::db::{DatabaseConnectionHelper, OptionalResult};
use crate::helpers::db_pagination::Paginate;
use crate::helpers::http::QueryParams;
use crate::helpers::time::current_timestamp;
use crate::helpers::DBPool;
use crate::models::application::{Application, ApplicationStatus, ApplicationCreateForm, ApplicationUpdateForm};
use crate::results::app_result::FormatAppResult;
use crate::results::{AppPaginationResult, AppResult};
use crate::schema::applications;

pub struct ApplicationRepository;

impl ApplicationRepository {
    pub fn list(&mut self, pool: &DBPool, q: QueryParams) -> AppPaginationResult<Application> {
        let search_format = format!("%{}%", q.get_search_query());
        applications::table
            .filter(
                applications::name
                    .ilike(search_format.clone())
                    .or(applications::url.ilike(search_format.clone()))
                    .or(applications::description.ilike(search_format)),
            )
            .filter(applications::deleted_at.is_null())
            .order_by(applications::created_at.desc())
            .paginate(q.get_page())
            .per_page(q.get_per_page())
            .load_and_count_pages::<Application>(&mut pool.conn())
            .into_app_result()
    }

    pub fn create(
        &mut self,
        pool: &DBPool,
        created_by: Uuid,
        data: ApplicationCreateForm,
    ) -> AppResult<Application> {
        diesel::insert_into(applications::dsl::applications)
            .values(Application {
                application_id: Uuid::new_v4(),
                created_by,
                name: data.name,
                code: data.code,
                url: data.url,
                webhook: data.webhook,
                description: data.description,
                logo: String::from(""),
                status: app_stringy_status(ApplicationStatus::Active)
                    .parse()
                    .unwrap(),
                created_at: current_timestamp(),
                updated_at: current_timestamp(),
                deleted_at: None,
            })
            .get_result::<Application>(&mut pool.conn())
            .into_app_result()
    }

    pub fn update(&mut self, pool: &DBPool, id: Uuid, form: ApplicationUpdateForm) -> AppResult<Application> {
        let mut app = self.find_by_id(pool, id)?;
        app.name = form.name;
        app.code = form.code;
        app.url = form.url;
        app.webhook = form.webhook;
        app.description = form.description;
        app.save_changes::<Application>(&mut pool.conn())
            .into_app_result()
    }

    pub fn delete(&mut self, pool: &DBPool, id: Uuid) -> AppResult<Application> {
        let mut app = self.find_by_id(pool, id)?;
        app.deleted_at = Some(current_timestamp());
        app.save_changes::<Application>(&mut pool.conn())
            .into_app_result()
    }

    pub fn find_by_id(&mut self, pool: &DBPool, id: Uuid) -> AppResult<Application> {
        applications::table
            .filter(applications::application_id.eq(id))
            .filter(applications::deleted_at.is_null())
            .first::<Application>(&mut pool.conn())
            .required("application")
    }

    pub fn find_owned_by_id(&mut self, pool: &DBPool, id: Uuid, owner: Uuid) -> AppResult<Uuid> {
        applications::table
            .select(applications::application_id)
            .filter(applications::application_id.eq(id))
            .filter(applications::created_by.eq(owner))
            .filter(applications::deleted_at.is_null())
            .first::<Uuid>(&mut pool.conn())
            .required("application")
    }

    pub fn find_by_code(&mut self, pool: &DBPool, code: String) -> AppResult<Application> {
        applications::table
            .filter(applications::code.eq(code))
            .filter(applications::deleted_at.is_null())
            .first::<Application>(&mut pool.conn())
            .required("application")
    }
}

pub fn app_stringy_status(status: ApplicationStatus) -> &'static str {
    match status {
        ApplicationStatus::Active => "active",
        ApplicationStatus::Inactive => "inactive",
    }
}
