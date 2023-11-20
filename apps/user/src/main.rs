use std::env;

use actix_files::Files;
use actix_web::web::Data;
use actix_web::App;
use actix_web::HttpServer;
use env_logger::Env;
use log::info;

use core::app_setup::{load_environment_variables, make_app_state};
use core::http::kernel::{
    actix_default_service, register_middlewares, register_routes, setup_cors, setup_logger,
};

use crate::http::controllers::routes;

mod http;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_environment_variables("user");

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let host: String = env::var("SERVER_HOST").unwrap();
    let port: u16 = env::var("SERVER_PORT").unwrap().parse().unwrap();
    let workers: usize = env::var("SERVER_WORKERS").unwrap().parse().unwrap();

    info!("starting server at http://localhost:{}", port);

    let app_state = make_app_state().await;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(app_state.clone()))
            .service(Files::new("/resources/static", "./resources/static"))
            .configure(|cfg| register_routes(cfg, routes()))
            .configure(register_middlewares)
            .wrap(setup_logger())
            .wrap(setup_cors(app_state.allowed_origins.clone()))
            .default_service(actix_default_service())
    })
    .shutdown_timeout(1)
    .bind((host, port))?
    .workers(workers)
    .run()
    .await
}
