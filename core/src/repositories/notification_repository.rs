use std::ops::DerefMut;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::helpers::db::{current_timestamp, OptionalResult};
use crate::helpers::db_pagination::{Paginate, PaginationResult};
use crate::helpers::get_db_conn;
use crate::helpers::http::QueryParams;
use crate::models::notification::{Notification, NotificationStatus};
use crate::models::DBPool;
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
            .get_result::<Notification>(get_db_conn(pool).deref_mut())
            .into_app_result()
    }

    pub fn list_paginated_by_user_id(
        &mut self,
        pool: &DBPool,
        id: Uuid,
        query_params: QueryParams,
    ) -> AppResult<PaginationResult<Notification>> {
        notifications::table
            .filter(notifications::receiver_id.eq(id))
            .filter(notifications::deleted_at.is_null())
            .order_by(notifications::updated_at.desc())
            .paginate(query_params.get_page())
            .per_page(query_params.get_per_page())
            .load_and_count_pages::<Notification>(get_db_conn(pool).deref_mut())
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
            .first::<Notification>(get_db_conn(pool).deref_mut())
            .required("notification")
    }
}
