use actix_web::web::{block, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, post, HttpRequest};
use uuid::Uuid;

use crate::enums::app_message::AppMessage;
use crate::helpers::http::QueryParams;
use crate::helpers::request::RequestHelper;
use crate::models::personal_access_token::PatCreateForm;
use crate::repositories::personal_access_token_repository::PersonaAccessTokenRepository;
use crate::results::http_result::ActixBlockingResultResponder;
use crate::results::HttpResult;
use crate::services::personal_access_token_service::PersonalAccessTokenService;

pub fn setting_controller(cfg: &mut ServiceConfig) {
    cfg.service(list_personal_access_token);
    cfg.service(generate_personal_access_token);
    cfg.service(delete_personal_access_token);
}

#[get("personal-access-tokens")]
async fn list_personal_access_token(req: HttpRequest, q: Query<QueryParams>) -> HttpResult {
    let ctx = req.context();
    block(move || PersonaAccessTokenRepository.list(ctx.database(), ctx.auth_id(), q.0))
        .await
        .respond()
}

#[post("personal-access-tokens")]
async fn generate_personal_access_token(req: HttpRequest, form: Json<PatCreateForm>) -> HttpResult {
    let ctx = req.context();
    block(move || PersonalAccessTokenService.create(ctx.app(), ctx.auth_id(), form.0))
        .await
        .respond()
}

#[delete("personal-access-tokens/{id}")]
async fn delete_personal_access_token(req: HttpRequest, id: Path<Uuid>) -> HttpResult {
    let ctx = req.context();
    block(move || {
        PersonalAccessTokenService
            .delete(ctx.database(), *id, Some(ctx.auth_id()))
            .map(|_| AppMessage::SuccessMessageStr("deleted"))
    })
    .await
    .respond()
}

