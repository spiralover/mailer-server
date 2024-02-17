use crate::models::Model;
use chrono::Utc;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Insertable, Queryable, AsChangeset, Identifiable, Clone)]
#[diesel(table_name = crate::schema::personal_access_tokens)]
#[diesel(primary_key(pat_id))]
pub struct PersonalAccessToken {
    pub pat_id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub comment: String,
    pub token: String,
    pub status: String,
    pub expired_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

impl PersonalAccessToken {
    pub fn is_active(&self) -> bool {
        self.status == PersonalAccessTokenStatus::Active.to_string()
    }

    pub fn has_expired(&self) -> bool {
        self.expired_at.le(&Utc::now().naive_utc())
    }

    pub fn is_usable(&self) -> bool {
        self.is_active() && !self.has_expired()
    }
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct PersonalAccessTokenMinimalData {
    pub pat_id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub comment: String,
    pub status: String,
    pub expired_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}

pub struct PatCreateDto {
    pub user_id: Uuid,
    pub title: String,
    pub comment: String,
    pub token: String,
    pub expired_at: chrono::NaiveDateTime,
}

#[derive(Deserialize)]
pub struct PatCreateForm {
    pub title: String,
    pub comment: String,
    pub expired_at: chrono::NaiveDateTime,
}

#[derive(Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum PersonalAccessTokenStatus {
    Active,
    Expired,
}

impl Model for PersonalAccessToken {}
