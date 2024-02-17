use std::ops::DerefMut;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SaveChangesDsl};
use uuid::Uuid;

use crate::helpers::db::{DatabaseConnectionHelper, OptionalResult};
use crate::helpers::db_pagination::Paginate;
use crate::helpers::http::QueryParams;
use crate::helpers::time::current_timestamp;
use crate::helpers::DBPool;
use crate::models::personal_access_token::{
    PatCreateDto, PersonalAccessToken, PersonalAccessTokenMinimalData, PersonalAccessTokenStatus,
};
use crate::results::app_result::FormatAppResult;
use crate::results::{AppPaginationResult, AppResult};
use crate::schema::personal_access_tokens;

pub struct PersonaAccessTokenRepository;

impl PersonaAccessTokenRepository {
    pub fn list(
        &mut self,
        pool: &DBPool,
        user_id: Uuid,
        q: QueryParams,
    ) -> AppPaginationResult<PersonalAccessTokenMinimalData> {
        personal_access_tokens::table
            .select((
                personal_access_tokens::pat_id,
                personal_access_tokens::user_id,
                personal_access_tokens::title,
                personal_access_tokens::comment,
                personal_access_tokens::status,
                personal_access_tokens::expired_at,
                personal_access_tokens::created_at,
            ))
            .filter(personal_access_tokens::deleted_at.is_null())
            .filter(personal_access_tokens::user_id.eq(user_id))
            .order_by(personal_access_tokens::created_at.desc())
            .paginate(q.get_page())
            .per_page(q.get_per_page())
            .load_and_count_pages::<PersonalAccessTokenMinimalData>(pool.conn().deref_mut())
            .into_app_result()
    }

    pub fn create(&mut self, pool: &DBPool, dto: PatCreateDto) -> AppResult<PersonalAccessToken> {
        let model = PersonalAccessToken {
            token: dto.token,
            pat_id: Uuid::new_v4(),
            user_id: dto.user_id,
            title: dto.title,
            comment: dto.comment,
            status: PersonalAccessTokenStatus::Active.to_string(),
            expired_at: dto.expired_at,
            created_at: current_timestamp(),
            updated_at: current_timestamp(),
            deleted_at: None,
        };

        diesel::insert_into(personal_access_tokens::dsl::personal_access_tokens)
            .values(model)
            .get_result::<PersonalAccessToken>(pool.conn().deref_mut())
            .into_app_result()
    }

    pub fn delete(
        &mut self,
        pool: &DBPool,
        id: Uuid,
        owner: Option<Uuid>,
    ) -> AppResult<PersonalAccessToken> {
        let mut pat = self.find_by_id(pool, id, owner)?;
        pat.deleted_at = Some(current_timestamp());
        pat.save_changes::<PersonalAccessToken>(pool.conn().deref_mut())
            .into_app_result()
    }

    pub fn find_by_id(
        &mut self,
        pool: &DBPool,
        id: Uuid,
        owner: Option<Uuid>,
    ) -> AppResult<PersonalAccessToken> {
        let query = personal_access_tokens::table
            .filter(personal_access_tokens::deleted_at.is_null())
            .filter(personal_access_tokens::pat_id.eq(id));

        if let Some(owner) = owner {
            return query
                .filter(personal_access_tokens::user_id.eq(owner))
                .first::<PersonalAccessToken>(pool.conn().deref_mut())
                .required("personal access token");
        }

        query
            .first::<PersonalAccessToken>(pool.conn().deref_mut())
            .required("personal access token")
    }

    pub fn find_by_token(
        &mut self,
        pool: &DBPool,
        token: String,
    ) -> AppResult<PersonalAccessToken> {
        personal_access_tokens::table
            .filter(personal_access_tokens::deleted_at.is_null())
            .filter(personal_access_tokens::token.eq(token))
            .first::<PersonalAccessToken>(pool.conn().deref_mut())
            .required("personal access token")
    }
}
