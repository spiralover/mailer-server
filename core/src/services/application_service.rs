use diesel::SaveChangesDsl;
use uuid::Uuid;

use crate::helpers::db::DatabaseConnectionHelper;
use crate::helpers::DBPool;
use crate::models::application::{Application, ApplicationStatus, ApplicationCreateForm, ApplicationUpdateForm};
use crate::repositories::application_repository::{app_stringy_status, ApplicationRepository};
use crate::results::app_result::FormatAppResult;
use crate::results::AppResult;

pub struct ApplicationService;

impl ApplicationService {
    pub fn create(
        &mut self,
        pool: &DBPool,
        created_by: Uuid,
        data: ApplicationCreateForm,
    ) -> AppResult<Application> {
        ApplicationRepository.create(pool, created_by, data)
    }

    pub fn update(&mut self, pool: &DBPool, id: Uuid, form: ApplicationUpdateForm) -> AppResult<Application> {
        ApplicationRepository.update(pool, id, form)
    }

    pub fn activate(&mut self, pool: &DBPool, id: Uuid) -> AppResult<Application> {
        let mut app = ApplicationRepository.find_by_id(pool, id)?;
        app.status = app_stringy_status(ApplicationStatus::Active).to_string();
        let updated = app.save_changes::<Application>(&mut pool.conn());
        updated.into_app_result()
    }

    pub fn deactivate(&mut self, pool: &DBPool, id: Uuid) -> AppResult<Application> {
        let mut app = ApplicationRepository.find_by_id(pool, id)?;
        app.status = app_stringy_status(ApplicationStatus::Inactive).to_string();
        let updated = app.save_changes::<Application>(&mut pool.conn());
        updated.into_app_result()
    }

    pub fn delete(&mut self, pool: &DBPool, id: Uuid) -> AppResult<Application> {
        ApplicationRepository.delete(pool, id)
    }
}
