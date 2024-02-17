use std::sync::{Arc, Mutex};
use actix_web::{App, HttpServer};
use env_logger::Env;
use log::info;

use cosmic::app_setup::{load_environment_variables, make_app_state, get_server_host_config, get_worker_configs, make_thread_name};
use cosmic::http::kernel::{actix_default_service, register_routes, setup_logger};

use crate::background_service::create_background_service;
use crate::http::routes;

mod background_service;
mod http;
mod queue_handler;
mod redis_error_handler;
mod schema;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_environment_variables("executor");

    let (host, port) = get_server_host_config();
    let (tasks_per_worker, workers) = get_worker_configs();
    let server_workers = workers.len();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    info!("starting server at http://localhost:{}", port);

    let app_state = make_app_state().await;
    let worker_count = Arc::new(Mutex::new(0));

    HttpServer::new(move || {
        let (_index, name) = make_thread_name(worker_count.clone(), workers.clone());
        create_background_service(&app_state.clone(), name, tasks_per_worker);

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
