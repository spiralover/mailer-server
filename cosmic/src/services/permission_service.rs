use crate::helpers::DBPool;
use crate::models::permission::Permission;
use crate::repositories::permission_repository::PermissionRepository;
use crate::results::AppResult;
use uuid::Uuid;

pub struct PermissionService;

impl PermissionService {
    pub fn create(
        &mut self,
        pool: &DBPool,
        created_by: Uuid,
        name: String,
        guard: String,
    ) -> AppResult<Permission> {
        PermissionRepository.create(pool, created_by, name, guard)
    }
}
