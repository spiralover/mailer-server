use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::mail_addresses)]
#[diesel(primary_key(mail_address_id))]
pub struct MailAddress {
    pub mail_address_id: Uuid,
    pub mail_id: Uuid,
    pub name: String,
    pub email: String,
    pub addr_type: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct MailAddressesSorted {
    pub(crate) cc: Vec<MailAddress>,
    pub(crate) bcc: Vec<MailAddress>,
    pub(crate) reply_to: Vec<MailAddress>,
    pub(crate) receivers: Vec<MailAddress>,
}

#[derive(Clone, PartialEq, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum MailAddressType {
    Cc,
    Bcc,
    ReplyTo,
    Receiver,
}
