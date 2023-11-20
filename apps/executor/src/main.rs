use std::env;

use actix_web::{App, HttpServer};
use env_logger::Env;
use log::info;

use core::app_setup::{load_environment_variables, make_app_state};
use core::http::kernel::{actix_default_service, register_routes, setup_logger};

use crate::background_service::create_background_service;
use crate::http::routes;
use crate::thread_namer::remove_name_lock_file;

mod background_service;
mod http;
mod queue_handler;
mod redis_error_handler;
mod schema;
mod service;
mod thread_namer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_environment_variables("executor");

    let host: String = env::var("SERVER_HOST").unwrap();
    let port: u16 = env::var("SERVER_PORT").unwrap().parse().unwrap();

    let server_workers: usize = env::var("SERVER_WORKERS").unwrap().parse().unwrap();
    let features_per_worker: i8 = env::var("SERVER_FEATURES_PER_WORKER")
        .unwrap()
        .parse()
        .unwrap();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    info!("starting server at http://localhost:{}", port);

    remove_name_lock_file();

    let app_state = make_app_state().await;

    HttpServer::new(move || {
        create_background_service(&app_state.clone(), features_per_worker);

        App::new()
            .app_data(app_state.clone())
            .configure(|cfg| register_routes(cfg, routes()))
            .wrap(setup_logger())
            .default_service(actix_default_service())
    })
    .workers(server_workers)
    .shutdown_timeout(1)
    .bind((host, port))?
    .run()
    .await
}
