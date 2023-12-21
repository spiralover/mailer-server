use diesel::dsl::not;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::helpers::db::{DatabaseConnectionHelper, OptionalResult};
use crate::helpers::db_pagination::{PageData, Paginate};
use crate::helpers::time::current_timestamp;
use crate::helpers::DBPool;
use crate::models::role::Role;
use crate::models::user_role::UserRole;
use crate::results::app_result::FormatAppResult;
use crate::results::AppResult;
use crate::schema::{roles, user_roles};

pub struct UserRoleRepository;

impl UserRoleRepository {
    pub fn create(
        &mut self,
        pool: &DBPool,
        created_by: Uuid,
        role_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<UserRole> {
        diesel::insert_into(user_roles::dsl::user_roles)
            .values(UserRole {
                user_role_id: Uuid::new_v4(),
                created_by,
                role_id,
                user_id,
                created_at: current_timestamp(),
                updated_at: current_timestamp(),
                deleted_at: None,
            })
            .get_result::<UserRole>(&mut pool.conn())
            .into_app_result()
    }

    pub fn list_by_user_id(&mut self, pool: &DBPool, id: Uuid) -> AppResult<Vec<UserRole>> {
        user_roles::table
            .filter(user_roles::user_id.eq(id))
            .filter(user_roles::deleted_at.is_null())
            .get_results::<UserRole>(&mut pool.conn())
            .into_app_result()
    }

    pub fn list_role_names_by_user_id(
        &mut self,
        pool: &DBPool,
        id: Uuid,
    ) -> AppResult<Vec<String>> {
        user_roles::table
            .inner_join(roles::table)
            .select(roles::role_name)
            .filter(user_roles::user_id.eq(id))
            .filter(user_roles::deleted_at.is_null())
            .get_results::<String>(&mut pool.conn())
            .into_app_result()
    }

    pub fn list_paginated_by_user_id(
        &mut self,
        pool: &DBPool,
        id: Uuid,
    ) -> AppResult<PageData<(UserRole, Role)>> {
        user_roles::table
            .filter(user_roles::user_id.eq(id))
            .filter(user_roles::deleted_at.is_null())
            .order_by(user_roles::updated_at.desc())
            .inner_join(roles::table)
            .paginate(1)
            .per_page(10)
            .load_and_count_pages::<(UserRole, Role)>(&mut pool.conn())
            .into_app_result()
    }

    pub fn list_assignable(&mut self, pool: &DBPool, user_id: Uuid) -> AppResult<Vec<Role>> {
        let user_role_query = user_roles::table
            .select(user_roles::role_id)
            .filter(user_roles::user_id.eq(user_id))
            .filter(user_roles::deleted_at.is_null());

        roles::table
            .filter(roles::deleted_at.is_null())
            .filter(not(roles::role_id.eq_any(user_role_query)))
            .get_results::<Role>(&mut pool.conn())
            .into_app_result()
    }

    pub fn find_by_id(&mut self, pool: &DBPool, id: Uuid) -> AppResult<UserRole> {
        user_roles::table
            .filter(user_roles::user_role_id.eq(id))
            .filter(user_roles::deleted_at.is_null())
            .first::<UserRole>(&mut pool.conn())
            .required("user role")
    }
}
