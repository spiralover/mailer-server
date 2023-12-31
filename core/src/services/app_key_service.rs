use diesel::SaveChangesDsl;
use uuid::Uuid;
use crate::helpers::db::DatabaseConnectionHelper;

use crate::models::app_key::AppKey;
use crate::helpers::DBPool;
use crate::repositories::app_key_repository::AppKeyRepository;
use crate::results::app_result::FormatAppResult;
use crate::results::AppResult;
use crate::results::http_result::ErroneousOptionResponse;

pub struct AppKeyService;

impl AppKeyService {
    pub fn generate(&mut self, pool: &DBPool, app_id: Uuid, created_by: Uuid) -> AppResult<AppKey> {
        let active_key_result = AppKeyRepository.find_active_by_app_id(pool, app_id);
        if !active_key_result.is_error_or_empty() {
            self.mark_key_as_expired(pool, active_key_result.unwrap().app_key_id)?;
        }

        AppKeyRepository.generate(pool, app_id, created_by)
    }

    pub fn mark_key_as_expired(&mut self, pool: &DBPool, id: Uuid) -> AppResult<AppKey> {
        let mut key = AppKeyRepository.find_by_id(pool, id)?;
        key.status = String::from("expired");
        key.save_changes::<AppKey>(&mut pool.conn()).into_app_result()
    }
}
