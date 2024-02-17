use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::helpers::db::{DatabaseConnectionHelper, OptionalResult};
use crate::helpers::db_pagination::{PageData, Paginate};
use crate::helpers::http::QueryParams;
use crate::helpers::time::current_timestamp;
use crate::helpers::DBPool;
use crate::models::notification::{Notification, NotificationStatus};
use crate::results::app_result::FormatAppResult;
use crate::results::AppResult;
use crate::schema::notifications;

pub struct NotificationRepository;

impl NotificationRepository {
    pub fn create(
        &mut self,
        pool: &DBPool,
        receiver_id: Uuid,
        title: String,
        url: String,
        content: String,
    ) -> AppResult<Notification> {
        let model = Notification {
            notification_id: Uuid::new_v4(),
            receiver_id,
            title,
            url,
            content,
            status: NotificationStatus::Unread.to_string(),
            created_at: current_timestamp(),
            updated_at: current_timestamp(),
            deleted_at: None,
        };

        diesel::insert_into(notifications::dsl::notifications)
            .values(model)
            .get_result::<Notification>(&mut pool.conn())
            .into_app_result()
    }

    pub fn list_paginated_by_user_id(
        &mut self,
        pool: &DBPool,
        id: Uuid,
        q: QueryParams,
    ) -> AppResult<PageData<Notification>> {
        notifications::table
            .filter(notifications::receiver_id.eq(id))
            .filter(notifications::deleted_at.is_null())
            .order_by(notifications::updated_at.desc())
            .paginate(q.get_page())
            .per_page(q.get_per_page())
            .load_and_count_pages::<Notification>(&mut pool.conn())
            .into_app_result()
    }

    pub fn find_by_id(
        &mut self,
        pool: &DBPool,
        id: Uuid,
        user_id: Uuid,
    ) -> AppResult<Notification> {
        notifications::table
            .filter(notifications::notification_id.eq(id))
            .filter(notifications::receiver_id.eq(user_id))
            .filter(notifications::deleted_at.is_null())
            .first::<Notification>(&mut pool.conn())
            .required("notification")
    }
}
