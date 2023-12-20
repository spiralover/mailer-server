use actix_web::web::{Redirect, ServiceConfig};
use actix_web::{get, HttpResponse};

use crate::helpers::responder::json_success_message;

pub fn main_controller_guest(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(about);
}

#[get("/")]
async fn index() -> Redirect {
    Redirect::to("https://spiralover.com")
}

#[get("about")]
async fn about() -> HttpResponse {
    json_success_message("About Page")
}
