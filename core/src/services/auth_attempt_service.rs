use crate::models::auth_attempt::{AuthAttempt, AuthAttemptStatus, CreateDto};
use crate::models::DBPool;
use crate::repositories::auth_attempt_repository::AuthAttemptRepository;
use crate::results::AppResult;

pub struct AuthAttemptService;

impl AuthAttemptService {
    pub fn create(
        &mut self,
        pool: &DBPool,
        status: AuthAttemptStatus,
        data: CreateDto,
    ) -> AppResult<AuthAttempt> {
        AuthAttemptRepository.create(pool, status, data)
    }
}
