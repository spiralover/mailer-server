use std::ops::DerefMut;

use diesel::dsl::not;
use diesel::{ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl, SaveChangesDsl};
use uuid::Uuid;

use crate::helpers::db::{current_timestamp, OptionalResult};
use crate::helpers::db_pagination::{Paginate, PaginationResult};
use crate::helpers::get_db_conn;
use crate::helpers::http::QueryParams;
use crate::models::permission::Permission;
use crate::models::role::{Role, RoleCreateForm, RoleStatus};
use crate::models::DBPool;
use crate::results::app_result::FormatAppResult;
use crate::results::AppResult;
use crate::roles::Roles;
use crate::schema::{permissions, role_permissions, roles};

pub struct RoleRepository;

impl RoleRepository {
    pub fn list(&mut self, pool: &DBPool, q: QueryParams) -> AppResult<PaginationResult<Role>> {
        roles::table
            .filter(roles::role_name.ilike(q.get_search_query_like()))
            .filter(roles::deleted_at.is_null())
            .order_by(roles::updated_at.desc())
            .paginate(q.get_page())
            .per_page(q.get_per_page())
            .load_and_count_pages::<Role>(get_db_conn(pool).deref_mut())
            .into_app_result()
    }

    pub fn list_assignable_permissions(
        &mut self,
        pool: &DBPool,
        id: Uuid,
    ) -> AppResult<Vec<Permission>> {
        let sq_existing_perms = role_permissions::table
            .select(role_permissions::permission_id)
            .filter(role_permissions::role_id.eq(id))
            .filter(role_permissions::deleted_at.is_null());

        permissions::table
            .filter(not(permissions::permission_id.eq_any(sq_existing_perms)))
            .filter(permissions::deleted_at.is_null())
            .get_results::<Permission>(get_db_conn(pool).deref_mut())
            .into_app_result()
    }

    pub fn create(
        &mut self,
        pool: &DBPool,
        created_by: Uuid,
        form: RoleCreateForm,
    ) -> AppResult<Role> {
        let model = Role {
            role_id: Uuid::new_v4(),
            created_by,
            role_name: form.name,
            guard_name: form.guard,
            status: RoleStatus::Active.to_string(),
            created_at: current_timestamp(),
            updated_at: current_timestamp(),
            deleted_at: None,
        };

        diesel::insert_into(roles::dsl::roles)
            .values(model)
            .get_result::<Role>(get_db_conn(pool).deref_mut())
            .into_app_result()
    }

    pub fn update(&mut self, pool: &DBPool, id: Uuid, data: RoleCreateForm) -> AppResult<Role> {
        let mut dept = self.find_by_id(pool, id)?;
        dept.role_name = data.name;
        dept.guard_name = data.guard;
        dept.save_changes::<Role>(get_db_conn(pool).deref_mut())
            .into_app_result()
    }

    pub fn find_by_id(&mut self, pool: &DBPool, id: Uuid) -> AppResult<Role> {
        roles::table
            .filter(roles::role_id.eq(id))
            .filter(roles::deleted_at.is_null())
            .first::<Role>(get_db_conn(pool).deref_mut())
            .required("role")
    }

    pub fn find_by_name(&mut self, pool: &DBPool, name: String) -> AppResult<Role> {
        roles::table
            .filter(roles::role_name.eq(name))
            .filter(roles::deleted_at.is_null())
            .first::<Role>(get_db_conn(pool).deref_mut())
            .required("role")
    }

    pub fn get_default_role_id(&mut self, pool: &DBPool) -> Uuid {
        RoleRepository
            .find_by_name(pool, Roles::User.to_string())
            .unwrap()
            .role_id
    }
}
