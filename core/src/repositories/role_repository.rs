use diesel::dsl::not;
use diesel::{ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl, SaveChangesDsl};
use uuid::Uuid;

use crate::enums::auth_role::AuthRole;
use crate::helpers::db::{DatabaseConnectionHelper, OptionalResult};
use crate::helpers::db_pagination::{PageData, Paginate};
use crate::helpers::http::QueryParams;
use crate::helpers::time::current_timestamp;
use crate::helpers::DBPool;
use crate::models::permission::Permission;
use crate::models::role::{Role, RoleCreateForm, RoleStatus};
use crate::results::app_result::FormatAppResult;
use crate::results::AppResult;
use crate::schema::{permissions, role_permissions, roles};

pub struct RoleRepository;

impl RoleRepository {
    pub fn list(&mut self, pool: &DBPool, q: QueryParams) -> AppResult<PageData<Role>> {
        roles::table
            .filter(roles::role_name.ilike(q.get_search_query_like()))
            .filter(roles::deleted_at.is_null())
            .order_by(roles::updated_at.desc())
            .paginate(q.get_page())
            .per_page(q.get_per_page())
            .load_and_count_pages::<Role>(&mut pool.conn())
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
            .get_results::<Permission>(&mut pool.conn())
            .into_app_result()
    }

    pub fn create(
        &mut self,
        pool: &DBPool,
        created_by: Uuid,
        form: RoleCreateForm,
    ) -> AppResult<Role> {
        diesel::insert_into(roles::dsl::roles)
            .values(Role {
                role_id: Uuid::new_v4(),
                created_by,
                role_name: form.name,
                guard_name: form.guard,
                status: RoleStatus::Active.to_string(),
                created_at: current_timestamp(),
                updated_at: current_timestamp(),
                deleted_at: None,
            })
            .get_result::<Role>(&mut pool.conn())
            .into_app_result()
    }

    pub fn update(&mut self, pool: &DBPool, id: Uuid, data: RoleCreateForm) -> AppResult<Role> {
        let mut dept = self.find_by_id(pool, id)?;
        dept.role_name = data.name;
        dept.guard_name = data.guard;
        dept.save_changes::<Role>(&mut pool.conn())
            .into_app_result()
    }

    pub fn find_by_id(&mut self, pool: &DBPool, id: Uuid) -> AppResult<Role> {
        roles::table
            .filter(roles::role_id.eq(id))
            .filter(roles::deleted_at.is_null())
            .first::<Role>(&mut pool.conn())
            .required("role")
    }

    pub fn find_by_name(&mut self, pool: &DBPool, name: String) -> AppResult<Role> {
        roles::table
            .filter(roles::role_name.eq(name))
            .filter(roles::deleted_at.is_null())
            .first::<Role>(&mut pool.conn())
            .required("role")
    }

    pub fn get_default_role_id(&mut self, pool: &DBPool) -> Uuid {
        RoleRepository
            .find_by_name(pool, AuthRole::Staff.to_string())
            .unwrap()
            .role_id
    }
}
