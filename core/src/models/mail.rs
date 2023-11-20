use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use uuid::Uuid;

use crate::helpers::http::HttpHeaderItem;

use super::super::schema::mails;

#[derive(
    Debug, Serialize, Deserialize, Insertable, Queryable, AsChangeset, Identifiable, Clone,
)]
#[diesel(table_name = mails)]
#[diesel(primary_key(mail_id))]
pub struct Mail {
    pub mail_id: Uuid,
    pub created_by: Uuid,
    pub application_id: Uuid,
    pub subject: String,
    pub message: String,
    pub from_name: String,
    pub from_email: String,
    pub reply_to_name: Option<String>,
    pub reply_to_email: Option<String>,
    pub trials: i16,
    pub status: String,
    pub sent_at: Option<chrono::NaiveDateTime>,
    pub next_retrial_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Clone, PartialEq, Display, Debug, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum MailStatus {
    Awaiting,
    Processing,
    Retrying,
    Failed,
    Sent,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MailData {
    pub subject: String,
    pub message: String,
    pub receiver: Vec<MailBox>,
    pub cc: Vec<MailBox>,
    pub bcc: Vec<MailBox>,
    pub reply_to: Vec<MailBox>,
    pub from: Option<MailBox>,
}

#[derive(Serialize, Deserialize)]
pub struct MailPayload {
    pub mails: Vec<MailData>,
}

#[derive(Serialize, Deserialize)]
pub struct ImpulseForwardablePayload {
    pub mail_name: String,
    pub mail_data: serde_json::Value,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MailQueueablePayload {
    pub created_by: Uuid,
    pub application_id: Uuid,

    pub subject: String,
    pub message: String,
    pub receiver: Vec<MailBox>,
    pub cc: Vec<MailBox>,
    pub bcc: Vec<MailBox>,
    pub reply_to: Vec<MailBox>,
    pub from: Option<MailBox>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MailSaved {
    pub mail: Mail,
    pub receiver: Vec<MailBox>,
    pub cc: Vec<MailBox>,
    pub bcc: Vec<MailBox>,
    pub reply_to: Vec<MailBox>,
}

#[derive(Serialize)]
pub struct MailCallbackPayload {
    pub reference: String,
    pub status_code: u16,
    pub response_body: String,
    pub headers: Vec<HttpHeaderItem>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MailBox {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct MailSuccessResponse {
    pub saved_mail: MailSaved,
    pub response_body: String,
}

#[derive(Serialize, Deserialize)]
pub struct MailFailureResponse {
    pub saved_mail: MailSaved,
    pub error_message: String,
}
