use actix_web::web::{Data, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, patch, post, put, HttpRequest};
use uuid::Uuid;

use core::app_state::AppState;
use core::auth::has_permission;
use core::enums::app_message::AppMessage;
use core::helpers::http::{IdPathParam, QueryParams};
use core::helpers::request::RequestHelper;
use core::models::application::{ApplicationCreateForm, ApplicationUpdateForm};
use core::models::mail::{MailPayload, MailQueueablePayload};
use core::permissions::Permissions;
use core::repositories::app_key_repository::AppKeyRepository;
use core::repositories::application_repository::ApplicationRepository;
use core::results::http_result::{ErroneousResponse, PaginationResponse, StructResponse};
use core::results::HttpResult;
use core::services::app_key_service::AppKeyService;
use core::services::application_service::ApplicationService;
use core::services::mail_service::MailService;

pub fn application_controller(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(show);
    cfg.service(store);
    cfg.service(update);
    cfg.service(delete);
    cfg.service(mails);
    cfg.service(deactivate);
    cfg.service(activate);
    cfg.service(keys);
    cfg.service(generate);
}

#[get("")]
async fn index(q: Query<QueryParams>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::ApplicationList, || {
        let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
        ApplicationRepository
            .list(db_pool, q.into_inner())
            .send_pagination_result()
    })
}

#[post("")]
async fn store(form: Json<ApplicationCreateForm>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::ApplicationCreate, || {
        let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
        ApplicationService
            .create(db_pool, req.auth_id(), form.into_inner())
            .send_struct_result()
    })
}

#[get("{id}")]
async fn show(id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    req.verify_user_permission(Permissions::ApplicationRead)?;
    let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
    ApplicationRepository
        .find_by_id(db_pool, id.to_owned())
        .send_result()
}

#[post("{id}/mails")]
async fn mails(id: Path<Uuid>, req: HttpRequest, form: Json<MailPayload>) -> HttpResult {
    req.verify_user_permission(Permissions::MailSend)?;
    let app = req.get_app_state();

    // Verify user has access to this neuron
    let app_id = ApplicationRepository.find_owned_by_id(req.get_db_pool(), *id, req.auth_id())?;

    for mail in form.mails.clone() {
        let _result = MailService.push_to_awaiting_queue(
            app,
            MailQueueablePayload {
                application_id: app_id,
                created_by: req.auth_id(),
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

    AppMessage::SuccessMessage("mail(s) queued").ok()
}

#[patch("{id}/activate")]
async fn activate(id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::ApplicationActivate, || {
        let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
        ApplicationService
            .activate(db_pool, id.to_owned())
            .send_result()
    })
}

#[patch("{id}/deactivate")]
async fn deactivate(id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::ApplicationDeactivate, || {
        let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
        ApplicationService
            .deactivate(db_pool, id.to_owned())
            .send_result()
    })
}

#[put("{id}")]
async fn update(form: Json<ApplicationUpdateForm>, id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::ApplicationUpdate, || {
        let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
        ApplicationService
            .update(db_pool, id.to_owned(), form.into_inner())
            .unwrap()
            .transform_response()
            .send_struct_result()
    })
}

#[delete("{id}")]
async fn delete(id: Path<Uuid>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::ApplicationDelete, || {
        let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
        ApplicationService
            .delete(db_pool, id.into_inner())
            .expect("Failed to delete application");

        AppMessage::SuccessMessage("application deleted").ok()
    })
}

#[get("{id}/keys")]
async fn keys(mut param: Path<IdPathParam>, req: HttpRequest) -> HttpResult {
    req.verify_user_permission(Permissions::ApplicationKeyList)?;
    let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
    let id = param.get_uuid()?;
    AppKeyRepository
        .find_active_by_app_id(db_pool, id)
        .send_result()
}

#[post("{id}/keys/generate")]
async fn generate(mut param: Path<IdPathParam>, req: HttpRequest) -> HttpResult {
    has_permission(req.to_owned(), Permissions::ApplicationKeyGenerate, || {
        let db_pool = req.app_data::<Data<AppState>>().unwrap().get_db_pool();
        let id = param.get_uuid()?;
        AppKeyService
            .generate(db_pool, id, req.auth_id())
            .send_result()
    })
}
