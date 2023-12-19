use std::ops::DerefMut;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::helpers::db::{OptionalResult};
use crate::helpers::time::current_timestamp;
use crate::helpers::get_db_conn;
use crate::models::mail::{Mail, MailQueueablePayload, MailStatus};
use crate::models::DBPool;
use crate::results::app_result::FormatAppResult;
use crate::results::AppResult;
use crate::schema::mails;

pub struct MailRepository;

impl MailRepository {
    pub fn create(&mut self, pool: &DBPool, payload: MailQueueablePayload) -> AppResult<Mail> {
        let from = payload.from.unwrap();
        let model = Mail {
            mail_id: Uuid::new_v4(),
            application_id: payload.application_id,
            subject: payload.subject,
            message: payload.message,
            from_name: from.name,
            from_email: from.email,
            reply_to_name: None,
            created_by: payload.created_by,
            trials: 0,
            status: MailStatus::Awaiting.to_string(),
            sent_at: None,
            next_retrial_at: None,
            created_at: current_timestamp(),
            updated_at: current_timestamp(),
            reply_to_email: None,
        };

        diesel::insert_into(mails::dsl::mails)
            .values(model)
            .get_result::<Mail>(get_db_conn(pool).deref_mut())
            .into_app_result()
    }

    pub fn find_by_id(&mut self, pool: &DBPool, id: Uuid) -> AppResult<Mail> {
        mails::table
            .filter(mails::mail_id.eq(id))
            .first::<Mail>(get_db_conn(pool).deref_mut())
            .required("mail")
    }
}
