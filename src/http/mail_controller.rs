use actix_web::{HttpResponse, post};
use actix_web::web::{Data, Json, ServiceConfig};

use crate::AppState;
use crate::core::http_responder::json_success_message;
use crate::core::mail_service::{MailPayload, MailService, push_mail_to_queue};

pub(crate) fn mail_controller(cfg: &mut ServiceConfig) {
    cfg.service(send);
}

#[post("mail/send")]
async fn send(mail: Json<MailPayload>, state: Data<AppState>) -> HttpResponse {
    let state = state.get_ref().clone().to_owned();

    for mail_data in &mail.mails {
        let client = state.redis.clone();
        let _ = push_mail_to_queue(client, MailService::new(mail.app.clone(), mail_data.clone()));
    }

    json_success_message("mail queued")
}
