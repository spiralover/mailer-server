use std::sync::Arc;
use tera::Context;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::models::announcement::{Announcement, AnnouncementCreateForm};
use crate::models::mail::MailBox;
use crate::models::user::FullName;
use crate::repositories::announcement_repository::AnnouncementRepository;
use crate::repositories::user_repository::UserRepository;
use crate::results::AppResult;
use crate::services::mailer_service::MailerService;

pub struct AnnouncementService;

impl AnnouncementService {
    pub fn send(
        &mut self,
        app: Arc<AppState>,
        sender_id: Uuid,
        form: AnnouncementCreateForm,
    ) -> AppResult<Announcement> {
        let pool = app.database();
        let announcement = AnnouncementRepository.create(pool, sender_id, form.clone())?;

        let users = UserRepository.all(pool)?;
        let receivers = users
            .iter()
            .map(|u| MailBox::new(u.full_name().as_str(), u.email.as_str()))
            .collect();

        let mut ctx = Context::new();
        ctx.insert("subject", &announcement.title);
        ctx.insert("message", &announcement.message);
        MailerService::new(app)
            .subject(format!("Announcement: {}", form.title.clone()))
            .view("message", ctx)
            .receivers(receivers)
            .for_each_recv()
            .send_silently();

        Ok(announcement)
    }
}
