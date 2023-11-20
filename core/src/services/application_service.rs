use std::ops::DerefMut;

use chrono::Utc;
use diesel::SaveChangesDsl;
use uuid::Uuid;

use crate::helpers::get_db_conn;
use crate::models::application::{
    Application, ApplicationCreateForm, ApplicationStatus, ApplicationUpdateForm,
};
use crate::models::DBPool;
use crate::repositories::application_repository::{
    application_stringy_status, ApplicationRepository,
};
use crate::results::app_result::FormatAppResult;
use crate::results::http_result::ErroneousOptionResponse;
use crate::results::AppResult;

pub struct ApplicationService;

impl ApplicationService {
    pub fn create(
        &mut self,
        pool: &DBPool,
        created_by: Uuid,
        mut data: ApplicationCreateForm,
    ) -> Application {
        data.code = Some(Utc::now().timestamp().to_string());
        ApplicationRepository.create(pool, created_by, data)
    }

    pub fn update(
        &mut self,
        pool: &DBPool,
        id: Uuid,
        form: ApplicationUpdateForm,
    ) -> AppResult<Application> {
        ApplicationRepository.update(pool, id, form)
    }

    pub fn activate(&mut self, pool: &DBPool, id: Uuid) -> AppResult<Application> {
        let result = ApplicationRepository.find_by_id(pool, id);
        if result.is_error_or_empty() {
            return result.get_error_result();
        }

        let mut app = result.unwrap();
        app.status = application_stringy_status(ApplicationStatus::Active).to_string();
        let updated = app.save_changes::<Application>(get_db_conn(pool).deref_mut());
        updated.into_app_result()
    }

    pub fn deactivate(&mut self, pool: &DBPool, id: Uuid) -> AppResult<Application> {
        let result = ApplicationRepository.find_by_id(pool, id);
        if result.is_error_or_empty() {
            return result.get_error_result();
        }

        let mut app = result.unwrap();
        app.status = application_stringy_status(ApplicationStatus::Inactive).to_string();
        let updated = app.save_changes::<Application>(get_db_conn(pool).deref_mut());
        updated.into_app_result()
    }

    pub fn delete(&mut self, pool: &DBPool, id: Uuid) -> AppResult<Application> {
        ApplicationRepository.delete(pool, id)
    }
}
