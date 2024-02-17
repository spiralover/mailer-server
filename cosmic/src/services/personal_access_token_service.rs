use std::sync::Arc;

use uuid::Uuid;

use crate::app_state::AppState;
use crate::helpers::hmac::hmac_generate_random;
use crate::helpers::DBPool;
use crate::models::personal_access_token::{PatCreateDto, PatCreateForm, PersonalAccessToken};
use crate::repositories::personal_access_token_repository::PersonaAccessTokenRepository;
use crate::results::AppResult;

pub struct PersonalAccessTokenService;

impl PersonalAccessTokenService {
    pub fn create(
        &mut self,
        app: Arc<AppState>,
        user_id: Uuid,
        dto: PatCreateForm,
    ) -> AppResult<PersonalAccessToken> {
        let token = hmac_generate_random();

        let prefix = app.auth_pat_prefix.clone();
        PersonaAccessTokenRepository.create(
            app.database(),
            PatCreateDto {
                user_id,
                title: dto.title,
                comment: dto.comment,
                expired_at: dto.expired_at,
                token: prefix + token.as_str(),
            },
        )
    }

    pub fn delete(
        &mut self,
        pool: &DBPool,
        id: Uuid,
        owner: Option<Uuid>,
    ) -> AppResult<PersonalAccessToken> {
        PersonaAccessTokenRepository.delete(pool, id, owner)
    }
}
