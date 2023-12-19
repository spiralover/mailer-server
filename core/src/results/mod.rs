use actix_web::HttpResponse;

use crate::enums::app_message::AppMessage;
use crate::helpers::db_pagination::PageData;

pub mod app_result;
pub mod http_result;
pub mod redis_result;

pub type AppResult<T> = Result<T, AppMessage>;
pub type AppPaginationResult<T> = Result<PageData<T>, AppMessage>;
pub type HttpResult = Result<HttpResponse, AppMessage>;
pub type RedisResult<T> = Result<T, redis::RedisError>;
