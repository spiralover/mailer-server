use actix_web::web::{block, Data, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, patch, post, put, HttpRequest};
use uuid::Uuid;
use crate::app_state::AppState;

use crate::enums::app_message::AppMessage;
use crate::enums::permissions::Permissions;
use crate::helpers::http::{IdPathParam, QueryParams};
use crate::helpers::request::RequestHelper;
use crate::helpers::DBPool;
use crate::models::application::{ApplicationCreateForm, ApplicationUpdateForm};
use crate::models::mail::{MailPayload, MailQueueablePayload};
use crate::repositories::app_key_repository::AppKeyRepository;
use crate::repositories::application_repository::ApplicationRepository;
use crate::results::http_result::ActixBlockingResultResponder;
use crate::results::HttpResult;
use crate::services::app_key_service::AppKeyService;
use crate::services::application_service::ApplicationService;
use crate::services::mail_service::MailService;

pub fn application_controller(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(show);
    cfg.service(store);
    cfg.service(update);
    cfg.service(mails);
    cfg.service(delete);
    cfg.service(deactivate);
    cfg.service(activate);
    cfg.service(keys);
    cfg.service(generate);
}

#[get("")]
async fn index(q: Query<QueryParams>, req: HttpRequest, pool: Data<DBPool>) -> HttpResult {
    req.verify_user_permission(Permissions::ApplicationList)?;
    block(move || ApplicationRepository.list(pool.get_ref(), q.into_inner()))
        .await
        .respond()
}

#[post("")]
async fn store(form: Json<ApplicationCreateForm>, req: HttpRequest, pool: Data<DBPool>) -> HttpResult {
    let auth_id = req.auth_id();
    req.verify_user_permission(Permissions::ApplicationKeyGenerate)?;
    block(move || ApplicationService.create(pool.get_ref(), auth_id, form.into_inner()))
        .await
        .respond()
}

#[get("{id}")]
async fn show(id: Path<Uuid>, req: HttpRequest, pool: Data<DBPool>) -> HttpResult {
    req.verify_user_permission(Permissions::ApplicationKeyGenerate)?;
    block(move || ApplicationRepository.find_by_id(pool.get_ref(), id.to_owned()))
        .await
        .respond()
}

#[post("{id}/mails")]
async fn mails(id: Path<Uuid>, req: HttpRequest, app: Data<AppState>, form: Json<MailPayload>) -> HttpResult {
    req.verify_user_permission(Permissions::MailSend)?;
    let auth_id = req.auth_id();

    block(move || {
        // Verify user has access to this neuron
        let app_id = ApplicationRepository.find_owned_by_id(&app.database().clone(), *id, auth_id)?;

        let total_mails = form.mails.len();
        for mail in form.mails.clone() {
            let _result = MailService.push_to_awaiting_queue(
                app.get_ref(),
                MailQueueablePayload {
                    application_id: app_id,
                    created_by: auth_id,
                    subject: mail.subject,
                    message: mail.message,
                    from: mail.from,
                    cc: mail.cc,
                    bcc: mail.bcc,
                    reply_to: mail.reply_to,
                    receiver: mail.receiver,
                },
            );
        }

        let message = Box::new(match total_mails <= 1 {
            true => format!("{} mail queued", total_mails),
            false => format!("{} mails queued", total_mails),
        });

        Ok(AppMessage::SuccessMessage(Box::leak(message)))
    })
        .await
        .respond()
}

#[patch("{id}/activate")]
async fn activate(id: Path<Uuid>, req: HttpRequest, pool: Data<DBPool>) -> HttpResult {
    req.verify_user_permission(Permissions::ApplicationKeyGenerate)?;
    block(move || ApplicationService.activate(pool.get_ref(), id.to_owned()))
        .await
        .respond()
}

#[patch("{id}/deactivate")]
async fn deactivate(id: Path<Uuid>, req: HttpRequest, pool: Data<DBPool>) -> HttpResult {
    req.verify_user_permission(Permissions::ApplicationKeyGenerate)?;
    block(move || ApplicationService.deactivate(pool.get_ref(), id.to_owned()))
        .await
        .respond()
}

#[put("{id}")]
async fn update(
    form: Json<ApplicationUpdateForm>,
    id: Path<Uuid>,
    req: HttpRequest,
    pool: Data<DBPool>,
) -> HttpResult {
    req.verify_user_permission(Permissions::ApplicationList)?;
    block(move || ApplicationService.update(pool.get_ref(), id.to_owned(), form.into_inner()))
        .await
        .respond()
}

#[delete("{id}")]
async fn delete(id: Path<Uuid>, req: HttpRequest, pool: Data<DBPool>) -> HttpResult {
    req.verify_user_permission(Permissions::ApplicationDelete)?;
    block(move || {
        ApplicationService
            .delete(pool.get_ref(), id.into_inner())
            .expect("Failed to delete application");

        Ok(AppMessage::SuccessMessage("application deleted"))
    })
        .await
        .respond()
}

#[get("{id}/keys")]
async fn keys(mut param: Path<IdPathParam>, req: HttpRequest, pool: Data<DBPool>) -> HttpResult {
    let id = param.get_uuid()?;
    req.verify_user_permission(Permissions::ApplicationKeyList)?;
    block(move || AppKeyRepository.find_active_by_app_id(pool.get_ref(), id))
        .await
        .respond()
}

#[post("{id}/keys/generate")]
async fn generate(
    mut param: Path<IdPathParam>,
    req: HttpRequest,
    pool: Data<DBPool>,
) -> HttpResult {
    let id = param.get_uuid()?;
    let auth_id = req.auth_id();
    req.verify_user_permission(Permissions::ApplicationKeyGenerate)?;
    block(move || AppKeyService.generate(pool.get_ref(), id, auth_id))
        .await
        .respond()
}
