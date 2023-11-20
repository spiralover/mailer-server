use std::ops::DerefMut;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SaveChangesDsl};
use uuid::Uuid;

use crate::helpers::db::{current_timestamp, OptionalResult};
use crate::helpers::db_pagination::{Paginate, PaginationResult};
use crate::helpers::get_db_conn;
use crate::helpers::http::QueryParams;
use crate::models::permission::Permission;
use crate::models::role_permission::RolePermission;
use crate::models::DBPool;
use crate::results::app_result::FormatAppResult;
use crate::results::AppResult;
use crate::schema::{permissions, role_permissions};

pub struct RolePermissionRepository;

impl RolePermissionRepository {
    pub fn create(
        &mut self,
        pool: &DBPool,
        created_by: Uuid,
        role_id: Uuid,
        permission_id: Uuid,
    ) -> AppResult<RolePermission> {
        let model = RolePermission {
            role_permission_id: Uuid::new_v4(),
            created_by,
            role_id,
            permission_id,
            created_at: current_timestamp(),
            updated_at: current_timestamp(),
            deleted_at: None,
        };

        diesel::insert_into(role_permissions::dsl::role_permissions)
            .values(model)
            .get_result::<RolePermission>(get_db_conn(pool).deref_mut())
            .into_app_result()
    }

    pub fn find_by_id(&mut self, pool: &DBPool, id: Uuid) -> AppResult<RolePermission> {
        role_permissions::table
            .filter(role_permissions::role_permission_id.eq(id))
            .first::<RolePermission>(get_db_conn(pool).deref_mut())
            .required("role permission")
    }

    pub fn remove(&mut self, pool: &DBPool, id: Uuid) -> AppResult<RolePermission> {
        let mut perm = RolePermissionRepository.find_by_id(pool, id)?;
        perm.deleted_at = Some(current_timestamp());
        perm.save_changes::<RolePermission>(get_db_conn(pool).deref_mut())
            .into_app_result()
    }

    pub fn list_paginated_by_role_id(
        &mut self,
        pool: &DBPool,
        role_id: Uuid,
        q: QueryParams,
    ) -> AppResult<PaginationResult<(RolePermission, Permission)>> {
        role_permissions::table
            .filter(role_permissions::role_id.eq(role_id))
            .inner_join(permissions::table)
            .paginate(q.get_page())
            .per_page(q.get_per_page())
            .load_and_count_pages::<(RolePermission, Permission)>(get_db_conn(pool).deref_mut())
            .into_app_result()
    }
}
