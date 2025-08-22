use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SaveChangesDsl};
use uuid::Uuid;

use crate::enums::app_message::AppMessage;
use crate::helpers::db::DatabaseConnectionHelper;
use crate::helpers::time::current_timestamp;
use crate::helpers::DBPool;
use crate::models::permission::UserPermissionItem;
use crate::models::role::{Role, RoleCreateForm, RoleStatus};
use crate::models::role_permission::RolePermission;
use crate::models::user::User;
use crate::models::user_permission::UserPermission;
use crate::models::user_role::UserRole;
use crate::repositories::role_repository::RoleRepository;
use crate::repositories::user_permission_repository::UserPermissionRepository;
use crate::repositories::user_repository::UserRepository;
use crate::repositories::user_role_repository::UserRoleRepository;
use crate::results::app_result::FormatAppResult;
use crate::results::AppResult;
use crate::schema::{permissions, role_permissions, user_permissions};
use crate::services::role_permission_service::RolePermissionService;
use crate::services::user_permission_service::UserPermissionService;

pub struct RoleService;

impl RoleService {
    pub fn create(
        &mut self,
        pool: &DBPool,
        created_by: Uuid,
        form: RoleCreateForm,
    ) -> AppResult<Role> {
        RoleRepository.create(pool, created_by, form)
    }

    pub fn update(&mut self, pool: &DBPool, id: Uuid, form: RoleCreateForm) -> AppResult<Role> {
        RoleRepository.update(pool, id, form)
    }

    pub fn add_permission(
        &mut self,
        pool: &DBPool,
        created_by: Uuid,
        role_id: Uuid,
        permission_id: Uuid,
    ) -> AppResult<RolePermission> {
        RolePermissionService.create(pool, created_by, role_id, permission_id)
    }

    pub fn assign_role_to_user(
        &mut self,
        pool: &DBPool,
        created_by: Uuid,
        role_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<UserRole> {
        UserRoleRepository.create(pool, created_by, role_id, user_id)
    }

    pub fn un_assign_user_role(
        &mut self,
        pool: &DBPool,
        user_role_id: Uuid,
    ) -> AppResult<AppMessage> {
        let mut ur = UserRoleRepository.find_by_id(pool, user_role_id)?;
        ur.deleted_at = Some(current_timestamp());
        ur.save_changes::<UserRole>(&mut pool.conn())
            .into_app_result()?;

        Ok(AppMessage::SuccessMessageStr("removed"))
    }

    pub fn user_permission_add(
        &mut self,
        pool: &DBPool,
        added_by: Uuid,
        user_id: Uuid,
        perm_id: Uuid,
    ) -> AppResult<UserPermission> {
        UserPermissionService.create(pool, added_by, perm_id, user_id)
    }

    pub fn user_permission_remove(
        &mut self,
        pool: &DBPool,
        user_perm_id: Uuid,
    ) -> AppResult<UserPermission> {
        UserPermissionRepository.remove(pool, user_perm_id)
    }

    pub fn activate(&mut self, pool: &DBPool, id: Uuid) -> AppResult<Role> {
        self.change_status(pool, id, RoleStatus::Active)
    }

    pub fn deactivate(&mut self, pool: &DBPool, id: Uuid) -> AppResult<Role> {
        self.change_status(pool, id, RoleStatus::Inactive)
    }

    pub fn list_user_permission_for_auth(
        &mut self,
        pool: &DBPool,
        id: Uuid,
        filter_name: Option<String>,
    ) -> Result<(User, Vec<UserPermissionItem>), AppMessage> {
        let user = UserRepository.find_by_id(pool, id)?;

        let role_ids: Vec<Uuid> = UserRoleRepository
            .list_by_user_id(pool, id)?
            .iter()
            .map(|ur| ur.role_id)
            .rev()
            .collect();

        let user_perm_query = user_permissions::table
            .select(user_permissions::permission_id)
            .filter(user_permissions::user_id.eq(id))
            .filter(user_permissions::deleted_at.is_null());

        let user_role_perm_query = role_permissions::table
            .select(role_permissions::permission_id)
            .filter(role_permissions::role_id.eq_any(role_ids))
            .filter(role_permissions::deleted_at.is_null());

        let query = permissions::table
            .select((
                permissions::permission_id,
                permissions::permission_name,
                permissions::guard_name,
            ))
            .filter(permissions::permission_id.eq_any(user_perm_query))
            .or_filter(permissions::permission_id.eq_any(user_role_perm_query));

        if let Some(name) = filter_name {
            return query
                .filter(permissions::permission_name.eq(name))
                .get_results::<UserPermissionItem>(&mut pool.conn())
                .into_app_result()
                .map(|perms| (user, perms));
        }

        query
            .get_results::<UserPermissionItem>(&mut pool.conn())
            .into_app_result()
            .map(|perms| (user, perms))
    }

    fn change_status(&mut self, pool: &DBPool, id: Uuid, status: RoleStatus) -> AppResult<Role> {
        let mut role = RoleRepository.find_by_id(pool, id)?;

        role.updated_at = current_timestamp();
        role.status = status.to_string();
        role.save_changes::<Role>(&mut pool.conn())
            .into_app_result()
    }
}
