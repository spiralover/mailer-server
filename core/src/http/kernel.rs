use std::rc::Rc;

use crate::helpers::responder::json_error_message_status;
use actix_cors::Cors;
use actix_web::http::{header, StatusCode};
use actix_web::middleware::Logger;
use actix_web::web::ServiceConfig;
use actix_web::{web, Route as ActixRoute};

use crate::http::middlewares::auth_middleware::Auth;

#[derive(Clone)]
pub struct Controller {
    pub path: String,
    pub handler: fn(cfg: &mut ServiceConfig),
}

#[derive(Clone)]
pub struct Route<T> {
    pub prefix: String,
    pub auth: Option<T>,
    pub controllers: Vec<Controller>,
}

pub fn register_routes(actix_config: &mut ServiceConfig, routes: Vec<Route<Auth>>) {
    log::debug!("discovering routes...");

    for route in routes {
        let route = Rc::new(route);
        for controller in &route.controllers {
            let path = route.prefix.as_str().to_owned() + controller.path.as_str();
            log::debug!(
                "route group: {}",
                if path.is_empty() { "/" } else { path.as_str() }
            );

            if path.is_empty() {
                actix_config.configure(controller.handler);
            } else if route.auth.is_some() {
                actix_config.service(
                    web::scope(path.as_str())
                        .wrap(route.auth.as_ref().cloned().unwrap())
                        .configure(controller.handler),
                );
            } else {
                actix_config.service(web::scope(path.as_str()).configure(controller.handler));
            }
        }
    }

    log::debug!("route discovery finished :)");
}

pub fn setup_logger() -> Logger {
    Logger::new("%{r}a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T")
        .exclude("/favicon.ico")
        .exclude("/system/docker-health-check")
}

pub fn setup_cors(origins: Vec<String>) -> Cors {
    let mut cors = Cors::default();

    for origin in origins {
        cors = cors.allowed_origin(origin.as_str());
    }

    cors.allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        .allowed_header(header::CONTENT_TYPE)
        .max_age(3600)
}

pub fn actix_default_service() -> ActixRoute {
    web::to(|| async { json_error_message_status("Resource(s) Not Found", StatusCode::NOT_FOUND) })
}

pub fn register_middlewares(_actix_config: &mut ServiceConfig) {
    // for middleware in middlewares() {
    // }
}
