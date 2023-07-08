use std::env;

use lettre::{Message, SmtpTransport, Transport};
use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::{Credentials};
use log::{error, info};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Receiver {
    email: String,
    name: String,
}

#[derive(Serialize, Deserialize)]
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
    pub fn send(&mut self, client: &SmtpTransport) {
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

        match client.send(&email) {
            Ok(response) => info!("Mail sent: {}", response.code()),
            Err(err) => error!("Failed to send mail: {}", err.to_string())
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