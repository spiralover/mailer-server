use std::collections::HashMap;
use std::env;
use std::sync::Arc;

use chrono::{Datelike, Utc};
use log::error;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use tera::Context;
use tokio::spawn;

use crate::app_state::AppState;
use crate::models::mail::MailBox;

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

#[derive(Serialize, Deserialize, Clone)]
pub struct MailerService {
    for_each_recv: bool,
    receiver: Vec<MailBox>,
    cc: Vec<MailBox>,
    bcc: Vec<MailBox>,
    reply_to: Vec<MailBox>,
    from: MailBox,
    subject: String,
    message: String,
}

#[derive(Serialize)]
struct MailPayload {
    app: String,
    mails: Vec<MailerService>,
}

impl Default for MailerService {
    fn default() -> Self {
        MailerService {
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
}

impl MailerService {
    pub fn new() -> Self {
        Self::default()
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

    pub fn view(&mut self, app: Arc<AppState>, file: &str, mut ctx: Context) -> &mut MailerService {
        ctx.insert("year", &Utc::now().year());
        ctx.insert("app_name", &app.app_name.clone());
        ctx.insert("app_desc", &app.app_desc.clone());
        ctx.insert("app_help_email", &app.app_help_email.clone());
        ctx.insert("app_frontend_url", &app.app_frontend_url.clone());

        self.body(app.render(file.to_string(), ctx))
    }

    pub fn send_silently(&mut self) {
        let mut mailer = self.clone();
        spawn(async move { mailer.send().await });
    }

    pub async fn send(&mut self) -> Result<MailerResponse, MailerError> {
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

    async fn do_send(&self) -> Result<MailerResponse, Error> {
        let client = reqwest::Client::new();
        let address = format!(
            "{}/api/v1/mail/send",
            env::var("MAILER_SERVER_ENDPOINT").unwrap()
        );

        let payload = match self.for_each_recv {
            true => {
                let mut mails = vec![];
                for receiver in &self.receiver {
                    let mut rec = self.clone();
                    rec.receiver = vec![receiver.clone()];
                    mails.push(rec);
                }

                MailPayload {
                    app: "mailer".to_string(),
                    mails,
                }
            }
            false => MailPayload {
                app: "mailer".to_string(),
                mails: vec![self.clone()],
            },
        };

        let resp = client
            .post(address)
            .json(&payload)
            .header(
                "X-Auth-Token",
                env::var("MAILER_SERVER_AUTH_TOKEN").unwrap(),
            )
            .send()
            .await?
            .json::<MailerResponse>()
            .await?;

        Ok(resp)
    }
}
