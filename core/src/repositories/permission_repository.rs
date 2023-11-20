use std::ops::DerefMut;

use diesel::{ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::helpers::db::{current_timestamp, OptionalResult};
use crate::helpers::db_pagination::{Paginate, PaginationResult};
use crate::helpers::get_db_conn;
use crate::helpers::http::QueryParams;
use crate::models::permission::Permission;
use crate::models::DBPool;
use crate::permissions::Permissions;
use crate::results::app_result::FormatAppResult;
use crate::results::AppResult;
use crate::schema::permissions;

pub struct PermissionRepository;

impl PermissionRepository {
    pub fn list(
        &mut self,
        pool: &DBPool,
        q: QueryParams,
    ) -> AppResult<PaginationResult<Permission>> {
        permissions::table
            .filter(permissions::permission_name.ilike(q.get_search_query_like()))
            .paginate(q.get_page())
            .per_page(q.get_per_page())
            .load_and_count_pages::<Permission>(get_db_conn(pool).deref_mut())
            .into_app_result()
    }

    pub fn create(
        &mut self,
        pool: &DBPool,
        created_by: Uuid,
        name: String,
        guard: String,
    ) -> AppResult<Permission> {
        let model = Permission {
            permission_id: Uuid::new_v4(),
            created_by,
            permission_name: name,
            guard_name: guard,
            created_at: current_timestamp(),
            updated_at: current_timestamp(),
            deleted_at: None,
        };

        diesel::insert_into(permissions::dsl::permissions)
            .values(model)
            .get_result::<Permission>(get_db_conn(pool).deref_mut())
            .into_app_result()
    }

    pub fn get_by_names(
        &mut self,
        pool: &DBPool,
        names: Vec<Permissions>,
    ) -> AppResult<Vec<Permission>> {
        let names: Vec<String> = names.iter().map(|p| p.to_string()).collect();
        permissions::table
            .filter(permissions::permission_name.eq_any(names))
            .get_results::<Permission>(get_db_conn(pool).deref_mut())
            .into_app_result()
    }

    pub fn find_by_name(&mut self, pool: &DBPool, name: String) -> AppResult<Permission> {
        permissions::table
            .filter(permissions::permission_name.eq(name))
            .first::<Permission>(get_db_conn(pool).deref_mut())
            .required("permission")
    }
}
