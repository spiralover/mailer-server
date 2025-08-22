use diesel::dsl::not;
use diesel::{ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl, SaveChangesDsl};
use uuid::Uuid;

use crate::helpers::db::{DatabaseConnectionHelper, OptionalResult};
use crate::helpers::db_pagination::{PageData, Paginate};
use crate::helpers::http::QueryParams;
use crate::helpers::time::current_timestamp;
use crate::helpers::DBPool;
use crate::models::permission::Permission;
use crate::models::user_permission::UserPermission;
use crate::results::app_result::FormatAppResult;
use crate::results::AppResult;
use crate::schema::{permissions, user_permissions};

pub struct UserPermissionRepository;

impl UserPermissionRepository {
    pub fn create(
        &mut self,
        pool: &DBPool,
        created_by: Uuid,
        permission_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<UserPermission> {
        let model = UserPermission {
            user_permission_id: Uuid::new_v4(),
            created_by,
            user_id,
            permission_id,
            created_at: current_timestamp(),
            updated_at: current_timestamp(),
            deleted_at: None,
        };

        diesel::insert_into(user_permissions::dsl::user_permissions)
            .values(model)
            .get_result::<UserPermission>(&mut pool.conn())
            .into_app_result()
    }

    pub fn list_paginated_by_user_id(
        &mut self,
        pool: &DBPool,
        id: Uuid,
        q: QueryParams,
    ) -> AppResult<PageData<(UserPermission, Permission)>> {
        user_permissions::table
            .filter(user_permissions::user_id.eq(id))
            .filter(permissions::permission_name.ilike(q.get_search_query_like()))
            .inner_join(permissions::table)
            .paginate(q.get_page())
            .per_page(q.get_per_page())
            .load_and_count_pages::<(UserPermission, Permission)>(&mut pool.conn())
            .into_app_result()
    }

    pub fn list_assignable(
        &mut self,
        pool: &DBPool,
        user_id: Uuid,
        q: QueryParams,
    ) -> AppResult<Vec<Permission>> {
        let user_role_query = user_permissions::table
            .select(user_permissions::permission_id)
            .filter(user_permissions::user_id.eq(user_id))
            .filter(user_permissions::deleted_at.is_null());

        permissions::table
            .filter(permissions::deleted_at.is_null())
            .filter(permissions::permission_name.ilike(q.get_search_query_like()))
            .filter(not(permissions::permission_id.eq_any(user_role_query)))
            .get_results::<Permission>(&mut pool.conn())
            .into_app_result()
    }

    pub fn find_by_id(&mut self, pool: &DBPool, id: Uuid) -> AppResult<UserPermission> {
        user_permissions::table
            .filter(user_permissions::user_permission_id.eq(id))
            .first::<UserPermission>(&mut pool.conn())
            .required("user permission")
    }

    pub fn remove(&mut self, pool: &DBPool, id: Uuid) -> AppResult<UserPermission> {
        let mut user_perm = UserPermissionRepository.find_by_id(pool, id)?;
        user_perm.deleted_at = Some(current_timestamp());
        user_perm
            .save_changes::<UserPermission>(&mut pool.conn())
            .into_app_result()
    }
}
