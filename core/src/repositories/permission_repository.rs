use diesel::{ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::enums::auth_permission::AuthPermission;
use crate::helpers::db::{DatabaseConnectionHelper, OptionalResult};
use crate::helpers::db_pagination::{PageData, Paginate};
use crate::helpers::http::QueryParams;
use crate::helpers::time::current_timestamp;
use crate::helpers::DBPool;
use crate::models::permission::Permission;
use crate::results::app_result::FormatAppResult;
use crate::results::AppResult;
use crate::schema::permissions;

pub struct PermissionRepository;

impl PermissionRepository {
    pub fn list(&mut self, pool: &DBPool, q: QueryParams) -> AppResult<PageData<Permission>> {
        permissions::table
            .filter(permissions::permission_name.ilike(q.get_search_query_like()))
            .paginate(q.get_page())
            .per_page(q.get_per_page())
            .load_and_count_pages::<Permission>(&mut pool.conn())
            .into_app_result()
    }

    pub fn create(
        &mut self,
        pool: &DBPool,
        created_by: Uuid,
        name: String,
        guard: String,
    ) -> AppResult<Permission> {
        diesel::insert_into(permissions::dsl::permissions)
            .values(Permission {
                permission_id: Uuid::new_v4(),
                created_by,
                permission_name: name,
                guard_name: guard,
                created_at: current_timestamp(),
                updated_at: current_timestamp(),
                deleted_at: None,
            })
            .get_result::<Permission>(&mut pool.conn())
            .into_app_result()
    }

    pub fn get_by_names(
        &mut self,
        pool: &DBPool,
        names: Vec<AuthPermission>,
    ) -> AppResult<Vec<Permission>> {
        let names: Vec<String> = names.iter().map(|p| p.to_string()).collect();
        permissions::table
            .filter(permissions::permission_name.eq_any(names))
            .get_results::<Permission>(&mut pool.conn())
            .into_app_result()
    }

    pub fn find_by_name(&mut self, pool: &DBPool, name: String) -> AppResult<Permission> {
        permissions::table
            .filter(permissions::permission_name.eq(name))
            .first::<Permission>(&mut pool.conn())
            .required("permission")
    }
}
