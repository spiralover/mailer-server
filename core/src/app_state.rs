use std::sync::{Arc, Mutex};

use lettre::SmtpTransport;
use redis::Client;
use tera::{Context, Tera};

use crate::helpers::DBPool;
use crate::models::mail::MailBox;

#[derive(Clone)]
pub struct AppState {
    pub app_name: String,
    pub app_desc: String,
    pub app_help_email: String,
    pub app_frontend_url: String,
    pub app_key: String,

    pub auth_token_lifetime: i64,
    pub auth_pat_prefix: String,

    pub mail_from: MailBox,
    pub mailer_application_id: String,
    pub mailer_system_user_id: String,

    pub max_image_upload_size: u64,
    pub tera: Tera,
    pub smtp: SmtpTransport,
    pub database: DBPool,
    pub redis: Client,
    pub max_retrials: i16,
    pub pulse_count: Arc<Mutex<i32>>,
    pub allowed_origins: Vec<String>,
    pub redis_queues: AppRedisQueues,
}

#[derive(Clone)]
pub struct AppRedisQueues {
    pub awaiting: String,
    pub processing: String,
    pub success: String,
    pub retrying: String,
    pub failure: String,
    pub callback: String,
}

impl AppState {
    pub fn database(&self) -> &DBPool {
        &self.database
    }

    pub fn title(&self, text: &str) -> String {
        format!("{} - {}", text, self.app_name)
    }

    pub fn render(&self, file: String, context: Context) -> String {
        let mut filename = file;
        if !filename.ends_with(".tera.html") {
            filename.push_str(".tera.html");
        }

        match self.tera.render(&filename, &context) {
            Ok(string) => string,
            Err(error) => panic!("{}", error),
        }
    }
}
