use std::ops::DerefMut;

use diesel::{ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::helpers::db::{current_timestamp, OptionalResult};
use crate::helpers::db_pagination::{Paginate, PaginationResult};
use crate::helpers::get_db_conn;
use crate::helpers::http::QueryParams;
use crate::models::announcement::{Announcement, AnnouncementCreateForm};
use crate::models::user::User;
use crate::models::DBPool;
use crate::results::app_result::FormatAppResult;
use crate::results::AppResult;
use crate::schema::{announcements, users};

pub struct AnnouncementRepository;

impl AnnouncementRepository {
    pub fn list(
        &mut self,
        pool: &DBPool,
        q: QueryParams,
    ) -> AppResult<PaginationResult<(Announcement, User)>> {
        announcements::table
            .filter(announcements::deleted_at.is_null())
            .filter(announcements::title.ilike(q.get_search_query_like()))
            .inner_join(users::table)
            .order_by(announcements::created_at.desc())
            .paginate(q.get_page())
            .per_page(q.get_per_page())
            .load_and_count_pages::<(Announcement, User)>(get_db_conn(pool).deref_mut())
            .into_app_result()
    }

    pub fn create(
        &mut self,
        pool: &DBPool,
        sender_id: Uuid,
        form: AnnouncementCreateForm,
    ) -> AppResult<Announcement> {
        let model = Announcement {
            announcement_id: Uuid::new_v4(),
            sender_id,
            title: form.title,
            message: form.message,
            created_at: current_timestamp(),
            updated_at: current_timestamp(),
            deleted_at: None,
        };

        diesel::insert_into(announcements::dsl::announcements)
            .values(model)
            .get_result::<Announcement>(get_db_conn(pool).deref_mut())
            .into_app_result()
    }

    pub fn find_by_id(&mut self, pool: &DBPool, id: Uuid) -> AppResult<(Announcement, User)> {
        announcements::table
            .filter(announcements::announcement_id.eq(id))
            .filter(announcements::deleted_at.is_null())
            .inner_join(users::table)
            .first::<(Announcement, User)>(get_db_conn(pool).deref_mut())
            .required("announcement")
    }
}
