use std::env;

use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use env_logger::Env;
use log::{info};
use redis::{Client};
use crate::core::database::create_database_connection;

use crate::core::mail_service::{create_smtp_client};
use crate::http::kernel::{actix_default_service, register_routes};
use crate::mailer::background_service::create_background_service;
use crate::mailer::thread_namer::remove_name_lock_file;

mod http;
mod core;
mod mailer;
mod models;
mod schema;

pub struct AppState {
    redis: Client,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let host: String = env::var("SERVER_HOST").unwrap();
    let port: u16 = env::var("SERVER_PORT").unwrap().parse().unwrap();

    let redis_url: String = env::var("REDIS_URL").unwrap();
    let redis_client = redis::Client::open(redis_url).unwrap();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    info!("starting server at http://localhost:{}", port);

    let smtp = create_smtp_client();
    let db_pool = create_database_connection();

    remove_name_lock_file();

    HttpServer::new(move || {
        create_background_service(&db_pool.clone(), redis_client.clone(), &smtp.clone(), 4);

        App::new()
            .app_data(Data::new(db_pool.clone()))
            .app_data(Data::new(AppState { redis: redis_client.clone() }))
            .configure(register_routes)
            // .wrap(middleware::NormalizePath::new(TrailingSlash::MergeOnly))
            .wrap(Logger::default())
            .default_service(actix_default_service())
    })
        .shutdown_timeout(1)
        .bind((host, port))?
        .run()
        .await
}
