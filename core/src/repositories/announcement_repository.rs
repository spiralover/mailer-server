use crate::helpers::db::{DatabaseConnectionHelper, OptionalResult};
use crate::helpers::db_pagination::Paginate;
use crate::helpers::http::QueryParams;
use crate::helpers::time::current_timestamp;
use diesel::{ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::helpers::DBPool;
use crate::models::announcement::{Announcement, AnnouncementCreateForm};
use crate::models::user::User;
use crate::results::app_result::FormatAppResult;
use crate::results::{AppPaginationResult, AppResult};
use crate::schema::{announcements, users};

pub struct AnnouncementRepository;

impl AnnouncementRepository {
    pub fn list(
        &mut self,
        pool: &DBPool,
        q: QueryParams,
    ) -> AppPaginationResult<(Announcement, User)> {
        announcements::table
            .filter(announcements::deleted_at.is_null())
            .filter(announcements::title.ilike(q.get_search_query_like()))
            .inner_join(users::table)
            .order_by(announcements::created_at.desc())
            .paginate(q.get_page())
            .per_page(q.get_per_page())
            .load_and_count_pages::<(Announcement, User)>(&mut pool.conn())
            .into_app_result()
    }

    pub fn create(
        &mut self,
        pool: &DBPool,
        sender_id: Uuid,
        form: AnnouncementCreateForm,
    ) -> AppResult<Announcement> {
        diesel::insert_into(announcements::dsl::announcements)
            .values(Announcement {
                announcement_id: Uuid::new_v4(),
                sender_id,
                title: form.title,
                message: form.message,
                created_at: current_timestamp(),
                updated_at: current_timestamp(),
                deleted_at: None,
            })
            .get_result::<Announcement>(&mut pool.conn())
            .into_app_result()
    }

    pub fn find_by_id(&mut self, pool: &DBPool, id: Uuid) -> AppResult<(Announcement, User)> {
        announcements::table
            .filter(announcements::announcement_id.eq(id))
            .filter(announcements::deleted_at.is_null())
            .inner_join(users::table)
            .first::<(Announcement, User)>(&mut pool.conn())
            .required("announcement")
    }
}
