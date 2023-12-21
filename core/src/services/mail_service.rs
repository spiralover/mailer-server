use std::ops::DerefMut;

use diesel::SaveChangesDsl;
use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::{Message, Transport};
use log::error;
use redis::Commands;
use serde::Serialize;

use crate::app_state::AppState;
use crate::helpers::get_db_conn;
use crate::models::mail::{Mail, MailBox, MailFailureResponse, MailStatus, MailSuccessResponse};
use crate::models::mail::{MailQueueablePayload, MailSaved};
use crate::models::mail_address::{MailAddress, MailAddressType};
use crate::models::DBPool;
use crate::repositories::mail_repository::MailRepository;
use crate::results::app_result::FormatAppResult;
use crate::results::{AppResult, RedisResult};
use crate::services::mail_address_service::MailAddressService;
use crate::services::mail_error_service::MailErrorService;

pub struct MailService;

impl MailService {
    pub fn create(&mut self, pool: &DBPool, payload: MailQueueablePayload) -> AppResult<MailSaved> {
        let mail = MailRepository.create(pool, payload.clone())?;
        let to_mailbox = |addr: MailAddress| MailBox::new(&addr.name, &addr.email);

        let mut cc = vec![];
        for mailbox in payload.cc {
            cc.push(to_mailbox(MailAddressService.create(
                pool,
                mail.mail_id,
                mailbox,
                MailAddressType::Cc,
            )?));
        }

        let mut bcc = vec![];
        for mailbox in payload.bcc {
            bcc.push(to_mailbox(MailAddressService.create(
                pool,
                mail.mail_id,
                mailbox,
                MailAddressType::Bcc,
            )?));
        }

        let mut reply_to = vec![];
        for mailbox in payload.reply_to {
            reply_to.push(to_mailbox(MailAddressService.create(
                pool,
                mail.mail_id,
                mailbox,
                MailAddressType::ReplyTo,
            )?));
        }

        let mut receiver = vec![];
        for mailbox in payload.receiver.clone() {
            receiver.push(to_mailbox(MailAddressService.create(
                pool,
                mail.mail_id,
                mailbox,
                MailAddressType::Receiver,
            )?));
        }

        Ok(MailSaved {
            mail,
            cc,
            bcc,
            reply_to,
            receiver,
        })
    }

    pub fn mark_as_success(
        &mut self,
        pool: &DBPool,
        response: MailSuccessResponse,
    ) -> AppResult<Mail> {
        let mut mail = MailRepository.find_by_id(pool, response.saved_mail.mail.mail_id)?;
        mail.status = MailStatus::Sent.to_string();
        mail.save_changes::<Mail>(get_db_conn(pool).deref_mut())
            .into_app_result()
    }

    fn log_failure(
        &mut self,
        pool: &DBPool,
        response: MailFailureResponse,
        status: MailStatus,
        increment_trials: bool,
    ) -> AppResult<Mail> {
        let mut mail = MailRepository.find_by_id(pool, response.saved_mail.mail.mail_id)?;

        if increment_trials {
            mail.trials += 1;
        }

        // update mail status
        if mail.status != status.to_string() {
            mail.status = status.to_string();

            mail.save_changes::<Mail>(get_db_conn(pool).deref_mut())
                .into_app_result()?;
        }

        // record mail error
        MailErrorService.create(pool, mail.mail_id, response.error_message)?;

        Ok(mail)
    }

    pub fn mark_as_failure(
        &mut self,
        pool: &DBPool,
        response: MailFailureResponse,
    ) -> AppResult<Mail> {
        self.log_failure(pool, response, MailStatus::Failed, true)
    }

    pub fn mark_as_retrying(
        &mut self,
        pool: &DBPool,
        response: MailFailureResponse,
    ) -> AppResult<Mail> {
        self.log_failure(pool, response, MailStatus::Retrying, true)
    }

    pub async fn send(&mut self, app: &AppState, thread_name: String, saved: MailSaved) {
        let subject = saved.mail.subject.clone();
        let smtp = app.smtp.clone();

        let make_mailbox = |rec: &MailBox| -> Mailbox {
            Mailbox::new(Some(rec.name.clone()), rec.email.parse().unwrap())
        };

        let mail_from = MailBox::new(&saved.mail.from_name, &saved.mail.from_email);

        let mut builder = Message::builder().from(make_mailbox(&mail_from));

        for receiver in &saved.receiver {
            builder = builder.to(make_mailbox(receiver))
        }

        for cc in &saved.cc {
            builder = builder.to(make_mailbox(cc))
        }

        for bcc in &saved.bcc {
            builder = builder.to(make_mailbox(bcc))
        }

        for reply_to in &saved.reply_to {
            builder = builder.to(make_mailbox(reply_to))
        }

        let email = builder
            .subject(saved.mail.subject.clone())
            .header(ContentType::TEXT_HTML)
            .body(saved.mail.message.clone())
            .unwrap();

        match smtp.send(&email) {
            Ok(_resp) => {
                let _ = self.push_to_success_notification_queue(
                    app,
                    MailSuccessResponse {
                        response_body: String::from("sent"),
                        saved_mail: saved.clone(),
                    },
                );
            }
            Err(err) => {
                error!(
                    "[{}] Failed to send mail #{}, [error: {}], re-queueing...",
                    thread_name,
                    subject,
                    err.to_string()
                );
                let _ = self.push_to_failure_notification_queue(
                    app,
                    MailFailureResponse {
                        saved_mail: saved.clone(),
                        error_message: err.to_string(),
                    },
                );
            }
        };
    }

    pub fn push_to_awaiting_queue(
        &mut self,
        app: &AppState,
        payload: MailQueueablePayload,
    ) -> RedisResult<i32> {
        self.push_to_queue(app, app.redis_queues.awaiting.clone(), payload)
    }

    pub fn push_to_processing_queue(
        &mut self,
        app: &AppState,
        saved: MailSaved,
    ) -> RedisResult<i32> {
        self.push_to_queue(app, app.redis_queues.processing.clone(), saved)
    }

    pub fn push_to_retrying_queue(&mut self, app: &AppState, saved: MailSaved) -> RedisResult<i32> {
        self.push_to_queue(app, app.redis_queues.retrying.clone(), saved)
    }

    pub fn push_to_failure_notification_queue(
        &mut self,
        app: &AppState,
        resp: MailFailureResponse,
    ) -> RedisResult<i32> {
        self.push_to_queue(app, app.redis_queues.failure.clone(), resp)
    }

    pub fn push_to_success_notification_queue(
        &mut self,
        app: &AppState,
        data: MailSuccessResponse,
    ) -> RedisResult<i32> {
        self.push_to_queue(app, app.redis_queues.success.clone(), data)
    }

    pub fn push_to_callback_queue(&mut self, app: &AppState, impulse: Mail) -> RedisResult<i32> {
        self.push_to_queue(app, app.redis_queues.callback.clone(), impulse)
    }

    fn push_to_queue<T: Serialize>(
        &mut self,
        app: &AppState,
        queue: String,
        data: T,
    ) -> RedisResult<i32> {
        let json = serde_json::to_string(&data).unwrap();
        app.clone()
            .redis
            .lpush::<&str, &str, i32>(&*queue, json.as_str())
    }
}
