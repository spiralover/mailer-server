#![allow(clippy::extra_unused_lifetimes)]

use derive_more::Display;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, VariantNames};
use uuid::Uuid;

use super::super::schema::notifications;

#[derive(
    Debug, Serialize, Deserialize, Insertable, Queryable, AsChangeset, Identifiable, Clone,
)]
#[diesel(table_name = notifications)]
#[diesel(primary_key(notification_id))]
pub struct Notification {
    pub notification_id: Uuid,
    pub receiver_id: Uuid,
    pub title: String,
    pub url: String,
    pub content: String,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Clone, Display, Debug, EnumString, VariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum NotificationStatus {
    Unread,
    Read,
    Glanced,
}
