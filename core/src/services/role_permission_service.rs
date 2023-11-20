use uuid::Uuid;

use crate::models::role_permission::RolePermission;
use crate::models::DBPool;
use crate::repositories::role_permission_repository::RolePermissionRepository;
use crate::results::AppResult;

pub struct RolePermissionService;

impl RolePermissionService {
    pub fn create(
        &mut self,
        pool: &DBPool,
        created_by: Uuid,
        role_id: Uuid,
        permission_id: Uuid,
    ) -> AppResult<RolePermission> {
        RolePermissionRepository.create(pool, created_by, role_id, permission_id)
    }

    pub fn remove(&mut self, pool: &DBPool, id: Uuid) -> AppResult<RolePermission> {
        RolePermissionRepository.remove(pool, id)
    }
}
