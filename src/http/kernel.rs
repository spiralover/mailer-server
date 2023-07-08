use actix_web::{HttpResponse, Route, web};
use actix_web::http::StatusCode;
use actix_web::web::ServiceConfig;
use crate::http::mail_controller::mail_controller;

pub(crate) fn actix_default_service() -> Route {
    web::to(|| async {
        HttpResponse::Ok()
            .status(StatusCode::NOT_FOUND)
            .body("Page Not Found")
    })
}

pub(crate) fn register_routes(cfg: &mut ServiceConfig) {
    let services = web::scope("api/v1").configure(mail_controller);
    cfg.service(services);
}
