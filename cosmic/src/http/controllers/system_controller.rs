use actix_web::get;
use actix_web::web::ServiceConfig;

use crate::enums::app_message::AppMessage;
use crate::results::HttpResult;

pub fn system_controller(cfg: &mut ServiceConfig) {
    cfg.service(docker_test);
}

#[get("docker-health-check")]
async fn docker_test() -> HttpResult {
    AppMessage::SuccessMessageStr("received").ok()
}
