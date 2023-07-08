use std::env;
use std::ops::Deref;

use lettre::{Message, SmtpTransport, Transport};
use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use log::{debug, error};
use redis::{Client, Commands, RedisResult};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Receiver {
    email: String,
    name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct MailService {
    receiver: Vec<Receiver>,
    cc: Vec<Receiver>,
    bcc: Vec<Receiver>,
    reply_to: Vec<Receiver>,
    from: Receiver,
    subject: String,
    message: String,
}

impl MailService {
    pub fn send(&mut self, redis: Client, smtp: &SmtpTransport, thread_name: String) {
        let make_mailbox = |rec: &Receiver| -> Mailbox {
            Mailbox::new(Some(rec.name.clone()), rec.email.parse().unwrap())
        };

        let mut builder = Message::builder()
            .from(make_mailbox(&self.from));

        for receiver in &self.receiver {
            builder = builder.to(make_mailbox(receiver))
        }

        for cc in &self.cc {
            builder = builder.to(make_mailbox(cc))
        }

        for bcc in &self.bcc {
            builder = builder.to(make_mailbox(bcc))
        }

        for reply_to in &self.reply_to {
            builder = builder.to(make_mailbox(reply_to))
        }

        let email = builder
            .subject(self.subject.clone())
            .header(ContentType::TEXT_HTML)
            .body(self.message.clone())
            .unwrap();

        match smtp.send(&email) {
            Ok(_) => {},
            Err(err) => {
                let json = serde_json::to_string(&self).unwrap();
                error!("[{}] Failed to send email, [mail: {}]]; [error: {}]", thread_name, json,  err.to_string());
                let _ = push_mail_to_queue(redis, self.deref().clone());
            }
        };
    }
}

pub(crate) fn create_smtp_client() -> SmtpTransport {
    let host = env::var("MAIL_HOST").unwrap();
    let port: u16 = env::var("MAIL_PORT").unwrap().parse().unwrap();
    let username = env::var("MAIL_USERNAME").unwrap();
    let password = env::var("MAIL_PASSWORD").unwrap();
    let credentials = Credentials::new(username, password);
    let authentication: bool = env::var("MAIL_AUTHENTICATION").unwrap().parse().unwrap();

    let builder = SmtpTransport::builder_dangerous(host.as_str()).port(port);

    if authentication {
        return builder.credentials(credentials).build();
    }

    builder.build()
}

pub(crate) fn push_mail_to_queue(mut client: Client, mail: MailService) -> RedisResult<i32> {
    let json = serde_json::to_string(&mail).unwrap();
    client.lpush::<&str, &str, i32>("queue:mailer:mails", json.as_str())
}