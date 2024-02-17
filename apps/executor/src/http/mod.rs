use crate::http::system_controller::system_controller;
use cosmic::http::kernel::{Controller, Route};
use cosmic::http::middlewares::auth_middleware::AuthMiddleware;

pub(crate) mod system_controller;

pub fn routes() -> Vec<Route<AuthMiddleware>> {
    let routes = vec![Route {
        auth: None,
        prefix: String::from("/system"),
        controllers: vec![Controller {
            path: String::from(""),
            handler: system_controller,
        }],
    }];

    routes
}
