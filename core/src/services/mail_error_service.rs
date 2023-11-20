use crate::models::mail_error::MailError;
use crate::models::DBPool;
use crate::repositories::mail_error_repository::MailErrorRepository;
use crate::results::AppResult;
use uuid::Uuid;

pub struct MailErrorService;

impl MailErrorService {
    pub fn create(
        &mut self,
        pool: &DBPool,
        mail_id: Uuid,
        smtp_error: String,
    ) -> AppResult<MailError> {
        MailErrorRepository.create(pool, mail_id, smtp_error)
    }
}
