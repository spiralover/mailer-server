use actix_web::HttpResponse;

use crate::enums::app_message::AppMessage;

pub mod app_result;
pub mod http_result;
pub mod redis_result;

pub type AppResult<T> = Result<T, AppMessage>;
pub type HttpResult = Result<HttpResponse, AppMessage>;
pub type RedisResult<T> = Result<T, redis::RedisError>;
