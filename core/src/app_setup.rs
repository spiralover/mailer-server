use std::env;
use std::path::Path;
use std::sync::{Arc, Mutex};

use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use lettre::SmtpTransport;
use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use log::info;
use redis::Client;
use tera::Tera;

use crate::app_state::{AppRedisQueues, AppState};
use crate::helpers::fs::get_cwd;
use crate::models::DBPool;
use crate::models::mail::MailBox;

pub async fn make_app_state() -> AppState {
    let db_url: String = env::var("DATABASE_DSN").unwrap();

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let database_pool: DBPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database pool.");

    // templating
    let tpl_dir = get_cwd() + "/resources/templates/**/*";
    let tera_templating = Tera::new(tpl_dir.as_str()).unwrap();

    // allowed origins
    let url_str = env::var("ALLOWED_ORIGINS").unwrap();
    let origins: Vec<&str> = url_str.split(',').collect();
    let origins: Vec<String> = origins.iter().map(|o| o.trim().to_string()).collect();

    // redis
    let redis_url: String = env::var("REDIS_DSN").unwrap();
    let redis_client = Client::open(redis_url).unwrap();

    AppState {
        tera: tera_templating,
        database: database_pool,
        redis: redis_client,
        smtp: create_smtp_client(),
        pulse_count: Arc::new(Mutex::new(0)),
        allowed_origins: origins,
        app_name: env::var("APP_NAME").unwrap(),
        app_desc: env::var("APP_DESC").unwrap(),
        app_key: env::var("APP_KEY").unwrap(),
        mail_from: MailBox {
            email: env::var("MAIL_FROM_ADDRESS").unwrap(),
            name: env::var("MAIL_FROM_NAME").unwrap(),
        },
        app_help_email: env::var("APP_HELP_EMAIL").unwrap(),
        app_frontend_url: env::var("FRONTEND_ADDRESS").unwrap(),
        max_retrials: env::var("MAX_RETRIALS").unwrap().parse().unwrap(),
        app_token_lifetime: env::var("AUTH_TOKEN_LIFETIME").unwrap().parse().unwrap(),
        max_image_upload_size: env::var("MAX_IMAGE_UPLOAD_SIZE").unwrap().parse().unwrap(),

        // redis
        redis_queues: AppRedisQueues {
            awaiting: env::var("REDIS_QUEUE_AWAITING").unwrap(),
            processing: env::var("REDIS_QUEUE_PROCESSING").unwrap(),
            retrying: env::var("REDIS_QUEUE_RETRYING").unwrap(),
            success: env::var("REDIS_QUEUE_SUCCESS").unwrap(),
            failure: env::var("REDIS_QUEUE_FAILURE").unwrap(),
            callback: env::var("REDIS_QUEUE_CALLBACK").unwrap(),
        },
    }
}

pub(crate) fn create_smtp_client() -> SmtpTransport {
    let host = env::var("MAIL_HOST").unwrap();
    let port: u16 = env::var("MAIL_PORT").unwrap().parse().unwrap();
    let username = env::var("MAIL_USERNAME").unwrap();
    let password = env::var("MAIL_PASSWORD").unwrap();
    let encryption = env::var("MAIL_ENCRYPTION").unwrap();
    let credentials = Credentials::new(username.clone(), password);

    info!("creating smtp client: smtp://{}:[password]@{}:{}", username, host.clone(), port.clone());

    SmtpTransport::builder_dangerous(host.as_str()).port(port).build();

    match encryption.as_str() {
        "local" => SmtpTransport::builder_dangerous(host.as_str())
            .port(port)
            .build(),
        "basic" => SmtpTransport::builder_dangerous(host.as_str())
            .port(port)
            .credentials(credentials)
            .build(),
        "startls" | "tls" => {
            SmtpTransport::starttls_relay(host.as_str())
                .unwrap()
                .credentials(credentials)
                .authentication(vec![Mechanism::Login])
                .build()
        }
        _ => {
            panic!("Encryption must be one of (local, basic, startls, tls)")
        }
    }
}

pub fn load_environment_variables(service: &str) {
    info!("root directory: {:?}", service);

    // load project level .env
    let path = format!("apps/{}/.env", service);
    dotenv::from_filename(path).ok();

    // load project level .env.main
    if Path::new(".env").exists() {
        info!("loading env file: .env");
        dotenv::from_filename(".env").ok();
    }

    // load project level .env.main
    if Path::new(".env.main").exists() {
        info!("loading env file: .env.main");
        dotenv::from_filename(".env.main").ok();
    }
}
