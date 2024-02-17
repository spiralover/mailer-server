use crate::results::AppResult;
use crate::models::user_permission::UserPermission;
use crate::helpers::DBPool;
use crate::repositories::user_permission_repository::UserPermissionRepository;
use uuid::Uuid;

pub struct UserPermissionService;

impl UserPermissionService {
    pub fn create(
        &mut self,
        pool: &DBPool,
        created_by: Uuid,
        permission_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<UserPermission> {
        UserPermissionRepository.create(pool, created_by, permission_id, user_id)
    }
}
