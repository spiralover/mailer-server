use std::ops::DerefMut;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::helpers::get_db_conn;
use crate::helpers::time::current_timestamp;
use crate::models::mail::MailBox;
use crate::models::mail_address::{MailAddress, MailAddressType, MailAddressesSorted};
use crate::models::DBPool;
use crate::results::app_result::FormatAppResult;
use crate::results::AppResult;
use crate::schema::mail_addresses;

pub struct MailAddressRepository;

impl MailAddressRepository {
    pub fn get_sorted(pool: &DBPool, mail_id: Uuid) -> AppResult<MailAddressesSorted> {
        let addresses = mail_addresses::table
            .filter(mail_addresses::mail_id.eq(mail_id))
            .get_results::<MailAddress>(get_db_conn(pool).deref_mut())
            .into_app_result()?;

        let filter = |addr_type: MailAddressType| -> Vec<MailAddress> {
            addresses
                .iter()
                .filter(|addr| addr.addr_type == addr_type.to_string())
                .cloned()
                .collect()
        };

        Ok(MailAddressesSorted {
            cc: filter(MailAddressType::Cc),
            bcc: filter(MailAddressType::Bcc),
            reply_to: filter(MailAddressType::ReplyTo),
            receivers: filter(MailAddressType::Receiver),
        })
    }

    pub fn create(
        &mut self,
        pool: &DBPool,
        mail_id: Uuid,
        mail_box: MailBox,
        addr_type: MailAddressType,
    ) -> AppResult<MailAddress> {
        let model = MailAddress {
            mail_id,
            mail_address_id: Uuid::new_v4(),
            name: mail_box.name,
            email: mail_box.email,
            addr_type: addr_type.to_string(),
            created_at: current_timestamp(),
        };

        diesel::insert_into(mail_addresses::dsl::mail_addresses)
            .values(model)
            .get_result::<MailAddress>(get_db_conn(pool).deref_mut())
            .into_app_result()
    }
}
