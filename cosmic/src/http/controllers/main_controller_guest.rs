use actix_web::get;
use actix_web::web::{Redirect, ServiceConfig};

pub fn main_controller_guest(cfg: &mut ServiceConfig) {
    cfg.service(index);
}

#[get("/")]
async fn index() -> Redirect {
    Redirect::to("https://spiralover.com")
}
