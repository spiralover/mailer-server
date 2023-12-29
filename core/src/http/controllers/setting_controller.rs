use crate::app_state::AppState;
use actix_web::web::{block, Data, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, post, HttpRequest};
use uuid::Uuid;

use crate::enums::app_message::AppMessage;
use crate::helpers::http::QueryParams;
use crate::helpers::request::RequestHelper;
use crate::helpers::DBPool;
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
async fn list_personal_access_token(
    req: HttpRequest,
    q: Query<QueryParams>,
    pool: Data<DBPool>,
) -> HttpResult {
    let auth_id = req.auth_id();
    block(move || PersonaAccessTokenRepository.list(pool.get_ref(), auth_id, q.0))
        .await
        .respond()
}

#[post("personal-access-tokens")]
async fn generate_personal_access_token(
    req: HttpRequest,
    app: Data<AppState>,
    form: Json<PatCreateForm>,
) -> HttpResult {
    let auth_id = req.auth_id();
    block(move || PersonalAccessTokenService.create(app.into_inner(), auth_id, form.0))
        .await
        .respond()
}

#[delete("personal-access-tokens/{id}")]
async fn delete_personal_access_token(
    req: HttpRequest,
    pool: Data<DBPool>,
    id: Path<Uuid>,
) -> HttpResult {
    let auth_id = req.auth_id();
    block(move || {
        PersonalAccessTokenService
            .delete(pool.get_ref(), *id, Some(auth_id))
            .map(|_| AppMessage::SuccessMessage("deleted"))
    })
    .await
    .respond()
}
