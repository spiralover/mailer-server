use diesel::{Insertable, Queryable, Selectable};
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::mail_errors)]
#[diesel(primary_key(mail_error_id))]
pub struct MailError {
    pub mail_error_id: Uuid,
    pub mail_id: Uuid,
    pub smtp_error: String,
    pub created_at: chrono::NaiveDateTime,
}
