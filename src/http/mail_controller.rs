use actix_web::{HttpResponse, post};
use actix_web::web::{Data, Json, ServiceConfig};
use redis::Commands;
use crate::AppState;
use crate::core::http_responder::json_success_message;
use crate::core::mail_service::MailService;

pub(crate) fn mail_controller(cfg: &mut ServiceConfig) {
    cfg.service(send);
}

#[post("mail/send")]
async fn send(mail: Json<MailService>, state: Data<AppState>) -> HttpResponse {
    let state = state.get_ref().clone().to_owned();
    let mut client = state.redis.clone();
    let json = serde_json::to_string(&mail.0).unwrap();
    let _ = client.lpush::<&str, &str, i32>("queue:mailer:mails", json.as_str());
    json_success_message("mail queued")
}
