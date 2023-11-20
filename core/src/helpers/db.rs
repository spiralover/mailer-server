use chrono::NaiveDateTime;
use diesel::result::Error;
use diesel::QueryResult;

use crate::enums::app_message::AppMessage;
use crate::enums::app_message::AppMessage::EntityNotFound;
use crate::results::app_result::AppOptionalResult;
use crate::results::AppResult;

pub fn current_timestamp() -> NaiveDateTime {
    chrono::Local::now().naive_local()
}

pub trait OptionalResult<'a, T> {
    fn optional(self) -> AppOptionalResult<T>;
    fn required(self, entity: &'a str) -> AppResult<T>;
}

impl<'a, T> OptionalResult<'a, T> for QueryResult<T> {
    fn optional(self) -> AppResult<Option<T>> {
        match self {
            Ok(value) => Ok(Some(value)),
            Err(Error::NotFound) => Ok(None),
            Err(e) => Err(AppMessage::DatabaseError(e.to_string())),
        }
    }

    fn required(self, entity: &'a str) -> AppResult<T> {
        match self {
            Ok(value) => Ok(value),
            Err(Error::NotFound) => Err(EntityNotFound(entity.to_string())),
            Err(e) => Err(AppMessage::DatabaseError(e.to_string())),
        }
    }
}
