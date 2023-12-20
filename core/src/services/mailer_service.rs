use std::collections::HashMap;
use std::env;
use std::str::FromStr;
use std::sync::Arc;

use chrono::{Datelike, Utc};
use log::error;
use serde::{Deserialize};
use tera::Context;
use tokio::spawn;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::models::mail::{MailBox, MailQueueablePayload};
use crate::results::RedisResult;
use crate::services::mail_service::MailService;

impl MailBox {
    pub fn new(name: &str, email: &str) -> MailBox {
        MailBox {
            name: name.to_string(),
            email: email.to_string(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct MailerResponse {
    pub code: i16,
    pub status: String,
    pub message: String,
    pub success: bool,
    pub data: HashMap<String, String>,
}

#[derive(Debug)]
pub struct MailerError {
    pub message: String,
}

#[derive(Clone)]
pub struct MailerService {
    app: Arc<AppState>,
    for_each_recv: bool,
    receiver: Vec<MailBox>,
    cc: Vec<MailBox>,
    bcc: Vec<MailBox>,
    reply_to: Vec<MailBox>,
    from: MailBox,
    subject: String,
    message: String,
}

impl MailerService {
    pub fn new(app: Arc<AppState>) -> Self {
        MailerService {
            app,
            for_each_recv: false,
            cc: vec![],
            bcc: vec![],
            reply_to: vec![],
            receiver: vec![],
            message: String::from(""),
            subject: String::from(""),
            from: MailBox {
                name: env::var("MAIL_FROM_NAME").unwrap(),
                email: env::var("MAIL_FROM_EMAIL").unwrap(),
            },
        }
    }

    pub fn subject(&mut self, s: String) -> &mut MailerService {
        self.subject = s;
        self
    }

    pub fn for_each_recv(&mut self) -> &mut MailerService {
        self.for_each_recv = true;
        self
    }

    pub fn receivers(&mut self, r: Vec<MailBox>) -> &mut MailerService {
        self.receiver = r;
        self
    }

    pub fn body(&mut self, b: String) -> &mut MailerService {
        self.message = b;
        self
    }

    pub fn view(&mut self, file: &str, mut ctx: Context) -> &mut MailerService {
        ctx.insert("year", &Utc::now().year());
        ctx.insert("app_name", &self.app.app_name.clone());
        ctx.insert("app_desc", &self.app.app_desc.clone());
        ctx.insert("app_help_email", &self.app.app_help_email.clone());
        ctx.insert("app_frontend_url", &self.app.app_frontend_url.clone());

        self.body(self.app.render(file.to_string(), ctx))
    }

    pub fn send_silently(&mut self) {
        let mut mailer = self.clone();
        spawn(async move { mailer.send().await });
    }

    pub async fn send(&mut self) -> Result<i32, MailerError> {
        let max_loop = 3;
        for i in 0..max_loop {
            let response = self.do_send().await;
            if let Ok(resp) = response {
                return Ok(resp);
            }

            let error = response.unwrap_err().to_string();

            error!("Error: {}", error);

            if i == max_loop {
                return Err(MailerError { message: error });
            }
        }

        // We shouldn't reach here, but let's make Rust happy :)
        Err(MailerError {
            message: String::from("Something went wrong"),
        })
    }

    async fn do_send(&self) -> RedisResult<i32> {
        let user_id = Uuid::from_str(self.app.mailer_system_user_id.as_str()).unwrap();
        let app_id = Uuid::from_str(self.app.mailer_application_id.as_str()).unwrap();
        MailService.push_to_awaiting_queue(
            self.app.as_ref(),
            MailQueueablePayload {
                application_id: app_id,
                created_by: user_id,
                subject: self.subject.clone(),
                message: self.message.clone(),
                from: Some(self.from.clone()),
                cc: self.cc.clone(),
                bcc: self.bcc.clone(),
                reply_to: self.reply_to.clone(),
                receiver: self.receiver.clone(),
            },
        )
    }
}
