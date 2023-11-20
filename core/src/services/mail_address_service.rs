use crate::models::mail::MailBox;
use crate::models::mail_address::{MailAddress, MailAddressType};
use crate::models::DBPool;
use crate::repositories::mail_address_repository::MailAddressRepository;
use crate::results::AppResult;
use uuid::Uuid;

pub struct MailAddressService;

impl MailAddressService {
    pub fn create(
        &mut self,
        pool: &DBPool,
        mail_id: Uuid,
        mail_box: MailBox,
        addr_type: MailAddressType,
    ) -> AppResult<MailAddress> {
        MailAddressRepository.create(pool, mail_id, mail_box, addr_type)
    }
}
