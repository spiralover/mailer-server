use actix_web::web::{block, Data, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, patch, post, put, HttpRequest};
use uuid::Uuid;

use crate::enums::app_message::AppMessage;
use crate::enums::auth_permission::AuthPermission;
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
async fn index(q: Query<QueryParams>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::ApplicationList)?;
        ApplicationRepository.list(ctx.database(), q.into_inner())
    })
    .await
    .respond()
}

#[post("")]
async fn store(form: Json<ApplicationCreateForm>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::ApplicationKeyGenerate)?;
        ApplicationService.create(ctx.database(), ctx.auth_id, form.into_inner())
    })
    .await
    .respond()
}

#[get("{id}")]
async fn show(id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::ApplicationKeyGenerate)?;
        ApplicationRepository.find_by_id(ctx.database(), id.to_owned())
    })
    .await
    .respond()
}

#[post("{id}/mails")]
async fn mails(id: Path<Uuid>, req: HttpRequest, form: Json<MailPayload>) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::MailSend)?;

        // Verify user has access to this neuron
        let app_id = ApplicationRepository.find_owned_by_id(ctx.database(), *id, ctx.auth_id())?;

        let total_mails = form.mails.len();
        for mail in form.mails.clone() {
            let _result = MailService.push_to_awaiting_queue(
                ctx.app().as_ref(),
                MailQueueablePayload {
                    application_id: app_id,
                    created_by: ctx.auth_id(),
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

        Ok(AppMessage::SuccessMessageStr(Box::leak(message)))
    })
    .await
    .respond()
}

#[patch("{id}/activate")]
async fn activate(id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::ApplicationKeyGenerate)?;
        ApplicationService.activate(ctx.database(), id.to_owned())
    })
    .await
    .respond()
}

#[patch("{id}/deactivate")]
async fn deactivate(id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::ApplicationKeyGenerate)?;
        ApplicationService.deactivate(ctx.database(), id.to_owned())
    })
    .await
    .respond()
}

#[put("{id}")]
async fn update(form: Json<ApplicationUpdateForm>, id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::ApplicationList)?;
        ApplicationService.update(ctx.database(), id.to_owned(), form.into_inner())
    })
    .await
    .respond()
}

#[delete("{id}")]
async fn delete(id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    block(move || {
        ctx.verify_user_permission(AuthPermission::ApplicationDelete)?;
        ApplicationService
            .delete(ctx.database(), id.into_inner())
            .expect("Failed to delete application");

        Ok(AppMessage::SuccessMessageStr("application deleted"))
    })
    .await
    .respond()
}

#[get("{id}/keys")]
async fn keys(mut param: Path<IdPathParam>, req: HttpRequest, pool: Data<DBPool>) -> HttpResult {
    let id = param.get_uuid()?;
    req.verify_user_permission(AuthPermission::ApplicationKeyList)?;
    block(move || AppKeyRepository.find_active_by_app_id(pool.get_ref(), id))
        .await
        .respond()
}

#[post("{id}/keys/generate")]
async fn generate(mut param: Path<IdPathParam>, req: HttpRequest) -> HttpResult {
    let ctx = req.context();
    let id = param.get_uuid()?;
    block(move || {
        ctx.verify_user_permission(AuthPermission::ApplicationKeyGenerate)?;
        AppKeyService.generate(ctx.database(), id, ctx.auth_id())
    })
    .await
    .respond()
}
