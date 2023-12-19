use std::str::FromStr;

use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::helpers::time::current_timestamp;

#[allow(dead_code)]
pub fn get_nullable_time(spent_at: Option<String>) -> NaiveDateTime {
    match spent_at {
        None => current_timestamp(),
        Some(val) => NaiveDateTime::parse_from_str(val.as_str(), "%Y-%m-%d %H:%M:%S").unwrap(),
    }
}

pub fn get_nullable_uuid(uuid: Option<String>) -> Option<Uuid> {
    uuid.map(|val| Uuid::from_str(val.as_str()).unwrap())
}

#[allow(dead_code)]
pub fn get_uuid_from_string(uuid: String) -> Uuid {
    Uuid::from_str(uuid.as_str()).unwrap()
}
