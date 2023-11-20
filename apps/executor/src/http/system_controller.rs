use actix_web::get;
use actix_web::web::ServiceConfig;

use core::enums::app_message::AppMessage;
use core::results::HttpResult;

pub(crate) fn system_controller(cfg: &mut ServiceConfig) {
    cfg.service(docker_test);
}

#[get("docker-health-check")]
async fn docker_test() -> HttpResult {
    AppMessage::SuccessMessage("received").ok()
}
