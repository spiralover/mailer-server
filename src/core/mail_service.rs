use std::env;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use diesel::RunQueryDsl;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use log::error;
use redis::{Client, Commands, RedisResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::database::{current_timestamp, DBPool, get_db_conn};
use crate::models::Mail;
use crate::schema::mails;

#[derive(Serialize, Deserialize, Clone)]
pub struct MailBox {
    email: String,
    name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct MailData {
    receiver: Vec<MailBox>,
    cc: Vec<MailBox>,
    bcc: Vec<MailBox>,
    reply_to: Vec<MailBox>,
    from: MailBox,
    subject: String,
    message: String,
}

#[derive(Deserialize, Clone)]
pub(crate) struct MailPayload {
    pub app: String,
    pub mails: Vec<MailData>,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct MailService {
    app: String,
    mail: MailData,
}

impl MailService {
    pub fn new(app: String, mail: MailData) -> MailService {
        MailService {
            app,
            mail,
        }
    }

    pub fn send(&mut self, pool: DBPool, redis: Client, smtp: &SmtpTransport, thread_name: String) {
        let make_mailbox = |rec: &MailBox| -> Mailbox {
            Mailbox::new(Some(rec.name.clone()), rec.email.parse().unwrap())
        };

        let mut builder = Message::builder()
            .from(make_mailbox(&self.mail.from));

        for receiver in &self.mail.receiver {
            builder = builder.to(make_mailbox(receiver))
        }

        for cc in &self.mail.cc {
            builder = builder.to(make_mailbox(cc))
        }

        for bcc in &self.mail.bcc {
            builder = builder.to(make_mailbox(bcc))
        }

        for reply_to in &self.mail.reply_to {
            builder = builder.to(make_mailbox(reply_to))
        }

        let email = builder
            .subject(self.mail.subject.clone())
            .header(ContentType::TEXT_HTML)
            .body(self.mail.message.clone())
            .unwrap();

        match smtp.send(&email) {
            Ok(_) => { store_to_database(&pool, self.clone()); }
            Err(err) => {
                let json = serde_json::to_string(&self).unwrap();
                error!("[{}] Failed to send email, [mail: {}]]; [error: {}]", thread_name, json,  err.to_string());
                let _ = push_mail_to_queue(redis, self.deref().clone());
            }
        };
    }
}

fn store_to_database(pool: &DBPool, mail: MailService) -> Mail {
    let cc = serde_json::Value::from_str(serde_json::to_string(&mail.mail.cc).unwrap().as_str());
    let bcc = serde_json::Value::from_str(serde_json::to_string(&mail.mail.bcc).unwrap().as_str());
    let mut receiver = mail.mail.receiver.get(0).unwrap();
    let model = Mail {
        id: Uuid::new_v4(),
        app: mail.app,
        subject: mail.mail.subject,
        message: mail.mail.message,
        receiver_name: receiver.name.clone(),
        receiver_email: receiver.email.clone(),
        reply_to_name: None,
        reply_to_email: None,
        cc: Some(cc.unwrap()),
        bcc: Some(bcc.unwrap()),
        sent_at: current_timestamp(),
    };

    diesel::insert_into(mails::dsl::mails)
        .values(model)
        .get_result::<Mail>(get_db_conn(pool).deref_mut())
        .unwrap()
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