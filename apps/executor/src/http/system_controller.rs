use actix_web::get;
use actix_web::web::ServiceConfig;

use cosmic::enums::app_message::AppMessage;
use cosmic::results::HttpResult;

pub(crate) fn system_controller(cfg: &mut ServiceConfig) {
    cfg.service(docker_test);
}

#[get("docker-health-check")]
async fn docker_test() -> HttpResult {
    AppMessage::SuccessMessageStr("received").ok()
}
