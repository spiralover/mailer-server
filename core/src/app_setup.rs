use std::env;
use std::path::Path;
use std::sync::{Arc, Mutex};

use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use lettre::SmtpTransport;
use log::info;
use redis::Client;
use tera::Tera;

use crate::app_state::{AppRedisQueues, AppServices, AppState};
use crate::helpers::fs::get_cwd;
use crate::models::mail::MailBox;
use crate::models::DBPool;
use crate::services::redis_service::RedisService;

pub async fn make_app_state() -> AppState {
    let database_pool = establish_database_connection();

    // templating
    let tpl_dir = get_cwd() + "/resources/templates/**/*";
    let tera_templating = Tera::new(tpl_dir.as_str()).unwrap();

    let redis = establish_redis_connection();

    AppState {
        app_name: env::var("MAILER_APP_NAME").unwrap(),
        app_desc: env::var("MAILER_APP_DESC").unwrap(),
        app_key: env::var("MAILER_APP_KEY").unwrap(),
        app_url: env::var("MAILER_APP_URL").unwrap(),
        app_logo_url: env::var("MAILER_APP_LOGO_URL").unwrap(),
        app_help_email: env::var("MAILER_APP_HELP_EMAIL").unwrap(),
        app_frontend_url: env::var("MAILER_FRONTEND_ADDRESS").unwrap(),

        auth_token_lifetime: env::var("MAILER_AUTH_TOKEN_LIFETIME")
            .unwrap()
            .parse()
            .unwrap(),
        auth_pat_prefix: env::var("MAILER_AUTH_PAT_PREFIX").unwrap(),

        mailer_application_id: env::var("MAILER_APPLICATION_ID").unwrap(),
        mailer_system_user_id: env::var("MAILER_SYSTEM_USER_ID").unwrap(),
        tera: tera_templating,
        database: database_pool.clone(),
        redis: redis.clone(),
        smtp: create_smtp_client(),
        pulse_count: Arc::new(Mutex::new(0)),
        allowed_origins: get_allowed_origins(),
        mail_from: MailBox {
            email: env::var("MAILER_MAIL_FROM_EMAIL").unwrap(),
            name: env::var("MAILER_MAIL_FROM_NAME").unwrap(),
        },
        max_retrials: env::var("MAILER_MAX_RETRIALS").unwrap().parse().unwrap(),
        max_image_upload_size: env::var("MAILER_MAX_IMAGE_UPLOAD_SIZE")
            .unwrap()
            .parse()
            .unwrap(),

        // redis
        redis_queues: get_redis_queues(),

        services: AppServices {
            redis: RedisService::new(redis),
        },
    }
}

pub fn get_server_host_config() -> (String, u16) {
    let host: String = env::var("MAILER_SERVER_HOST").unwrap();
    let port: u16 = env::var("MAILER_SERVER_PORT").unwrap().parse().unwrap();
    (host, port)
}

pub fn establish_redis_connection() -> Client {
    let redis_url: String = env::var("MAILER_REDIS_DSN").unwrap();
    Client::open(redis_url).unwrap()
}

pub fn establish_database_connection() -> DBPool {
    let db_url: String = env::var("MAILER_DATABASE_DSN").unwrap();
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database pool.")
}

pub fn get_redis_queues() -> AppRedisQueues {
    AppRedisQueues {
        awaiting: env::var("MAILER_REDIS_QUEUE_AWAITING").unwrap(),
        processing: env::var("MAILER_REDIS_QUEUE_PROCESSING").unwrap(),
        retrying: env::var("MAILER_REDIS_QUEUE_RETRYING").unwrap(),
        success: env::var("MAILER_REDIS_QUEUE_SUCCESS").unwrap(),
        failure: env::var("MAILER_REDIS_QUEUE_FAILURE").unwrap(),
        callback: env::var("MAILER_REDIS_QUEUE_CALLBACK").unwrap(),
    }
}

pub(crate) fn create_smtp_client() -> SmtpTransport {
    let host = env::var("MAILER_MAIL_HOST").unwrap();
    let port: u16 = env::var("MAILER_MAIL_PORT").unwrap().parse().unwrap();
    let username = env::var("MAILER_MAIL_USERNAME").unwrap();
    let password = env::var("MAILER_MAIL_PASSWORD").unwrap();
    let encryption = env::var("MAILER_MAIL_ENCRYPTION").unwrap();
    let credentials = Credentials::new(username.clone(), password);

    info!(
        "creating smtp client: smtp://{}:[password]@{}:{}",
        username,
        host.clone(),
        port.clone()
    );

    SmtpTransport::builder_dangerous(host.as_str())
        .port(port)
        .build();

    match encryption.as_str() {
        "local" => SmtpTransport::builder_dangerous(host.as_str())
            .port(port)
            .build(),
        "basic" => SmtpTransport::builder_dangerous(host.as_str())
            .port(port)
            .credentials(credentials)
            .build(),
        "startls" | "tls" => SmtpTransport::starttls_relay(host.as_str())
            .unwrap()
            .credentials(credentials)
            .authentication(vec![Mechanism::Login])
            .build(),
        _ => {
            panic!("Encryption must be one of (local, basic, startls, tls)")
        }
    }
}

pub fn get_allowed_origins() -> Vec<String> {
    let url_str = env::var("MAILER_ALLOWED_ORIGINS").unwrap();
    let origins: Vec<&str> = url_str.split(',').collect();
    origins.iter().map(|o| o.trim().to_string()).collect()
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

    // load project level .env.main
    let filename = format!(".env.{}", service);
    if Path::new(filename.as_str()).exists() {
        info!("loading env file: {}", filename);
        dotenv::from_filename(filename).ok();
    }
}

pub fn make_thread_name(worker_count: Arc<Mutex<usize>>, workers: Vec<String>) -> (usize, String) {
    let mut worker_index = worker_count.lock().unwrap();
    let thread_name = workers.get(worker_index.to_owned()).unwrap();
    *worker_index += 1;
    (*worker_index - 1, thread_name.to_owned())
}

pub fn get_worker_configs() -> (i8, Vec<String>) {
    let tasks_per_worker: i8 = env::var("MAILER_SERVER_TASKS_PER_WORKER")
        .unwrap()
        .parse()
        .unwrap();

    let workers: Vec<String> = env::var("MAILER_SERVER_WORKERS")
        .unwrap()
        .split(',')
        .map(|s| s.to_string())
        .collect();

    (tasks_per_worker, workers)
}
