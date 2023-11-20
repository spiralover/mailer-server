use crate::enums::app_message::AppMessage;
use crate::results::AppResult;
use crate::results::RedisResult;

pub trait FormatRedisResult<T> {
    fn into_app_result(self) -> AppResult<T>;
}

impl<T> FormatRedisResult<T> for RedisResult<T> {
    fn into_app_result(self) -> AppResult<T> {
        match self {
            Ok(value) => Ok(value),
            Err(err) => Err(AppMessage::RedisError(err)),
        }
    }
}
