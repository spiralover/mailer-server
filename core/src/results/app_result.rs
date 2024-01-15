use actix_web::error::BlockingError;
use diesel::result::Error;
use diesel::QueryResult;

use crate::enums::app_message::AppMessage;
use crate::results::AppResult;

pub type AppOptionalResult<T> = Result<Option<T>, AppMessage>;

pub trait FormatAppResult<T> {
    fn into_app_result(self) -> AppResult<T>;
}

pub trait ActixBlockResult<T> {
    fn into_app_result(self) -> AppResult<T>;
}

impl<T> FormatAppResult<T> for QueryResult<T> {
    fn into_app_result(self) -> AppResult<T> {
        match self {
            Ok(value) => Ok(value),
            Err(Error::NotFound) => Err(AppMessage::DatabaseEntityNotFound),
            Err(e) => Err(AppMessage::DatabaseErrorMessage(e.to_string())),
        }
    }
}

impl<T> ActixBlockResult<T> for Result<AppResult<T>, BlockingError> {
    fn into_app_result(self) -> AppResult<T> {
        match self {
            Ok(res) => res,
            Err(err) => Err(AppMessage::BlockingError(err)),
        }
    }
}

impl<T> ActixBlockResult<T> for Result<T, BlockingError> {
    fn into_app_result(self) -> AppResult<T> {
        match self {
            Ok(res) => Ok(res),
            Err(err) => Err(AppMessage::BlockingError(err)),
        }
    }
}
