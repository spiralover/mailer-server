use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::schema::announcements;

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Insertable,
    Queryable,
    Selectable,
    AsChangeset,
    Identifiable,
    Clone,
)]
#[diesel(table_name = announcements)]
#[diesel(primary_key(announcement_id))]
pub struct Announcement {
    pub announcement_id: Uuid,
    pub sender_id: Uuid,
    pub title: String,
    pub message: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AnnouncementCreateForm {
    pub title: String,
    pub message: String,
}
