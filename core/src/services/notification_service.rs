use diesel::SaveChangesDsl;
use uuid::Uuid;

use crate::helpers::db::DatabaseConnectionHelper;
use crate::helpers::DBPool;
use crate::models::notification::{Notification, NotificationStatus};
use crate::repositories::notification_repository::NotificationRepository;
use crate::results::app_result::FormatAppResult;
use crate::results::AppResult;

pub struct NotificationService;

impl NotificationService {
    #[allow(dead_code)]
    pub fn create(
        &mut self,
        pool: &DBPool,
        receiver_id: Uuid,
        title: String,
        url: String,
        content: String,
    ) -> AppResult<Notification> {
        NotificationRepository.create(pool, receiver_id, title, url, content)
    }

    pub fn mark_as_read(
        &mut self,
        pool: &DBPool,
        id: Uuid,
        user_id: Uuid,
    ) -> AppResult<Notification> {
        self.mark(pool, id, user_id, NotificationStatus::Read)
    }

    pub fn mark_as_glanced(
        &mut self,
        pool: &DBPool,
        id: Uuid,
        user_id: Uuid,
    ) -> AppResult<Notification> {
        self.mark(pool, id, user_id, NotificationStatus::Glanced)
    }

    fn mark(
        &mut self,
        pool: &DBPool,
        id: Uuid,
        user_id: Uuid,
        status: NotificationStatus,
    ) -> AppResult<Notification> {
        let mut notification = NotificationRepository.find_by_id(pool, id, user_id)?;
        notification.status = status.to_string();
        notification
            .save_changes::<Notification>(&mut pool.conn())
            .into_app_result()
    }
}
