use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::mails)]
pub struct Mail {
    pub id: Uuid,
    pub app: String,
    pub subject: String,
    pub message: String,
    pub receiver_name: String,
    pub receiver_email: String,
    pub reply_to_name: Option<String>,
    pub reply_to_email: Option<String>,
    pub cc: Option<serde_json::Value>,
    pub bcc: Option<serde_json::Value>,
    pub sent_at: chrono::NaiveDateTime,
}
