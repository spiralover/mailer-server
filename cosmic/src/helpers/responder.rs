use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::Serialize;
use std::fmt::{Display, Formatter};

use crate::helpers::db_pagination::PageData;

#[derive(Debug, Serialize)]
pub(crate) struct JsonResponseEmptyMessage {}

#[derive(Debug, Serialize)]
pub struct JsonResponse<T: Serialize> {
    pub(crate) code: u16,
    pub(crate) success: bool,
    pub(crate) status: String,
    pub(crate) message: Option<String>,
    pub(crate) data: T,
}

impl<T: Serialize> Display for JsonResponse<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(serde_json::to_string(self).unwrap().as_str())
    }
}

#[derive(Serialize)]
pub struct PaginationResponse<T: Serialize> {
    pub(crate) success: bool,
    pub(crate) total_pages: i64,
    pub(crate) total_records: i64,
    pub(crate) status: u16,
    pub(crate) data: Vec<T>,
}

pub fn json<T: Serialize>(data: T, status: StatusCode) -> HttpResponse {
    HttpResponse::build(status).json(data)
}

pub fn json_success<T: Serialize>(data: T, message: Option<String>) -> HttpResponse {
    json(
        JsonResponse {
            success: true,
            code: StatusCode::OK.as_u16(),
            status: StatusCode::OK.to_string(),
            data,
            message,
        },
        StatusCode::OK,
    )
}

pub fn json_pagination<T: Serialize>(data: PageData<T>) -> HttpResponse {
    json(
        PaginationResponse {
            success: true,
            status: 200,
            data: data.records,
            total_pages: data.total_pages,
            total_records: data.total_records,
        },
        StatusCode::OK,
    )
}

pub fn json_error<T: Serialize>(
    data: T,
    status: StatusCode,
    message: Option<String>,
) -> HttpResponse {
    json(
        JsonResponse {
            success: false,
            code: status.as_u16(),
            status: status.to_string(),
            data,
            message,
        },
        status,
    )
}

pub fn json_error_message(message: &str) -> HttpResponse {
    json_error_message_status(message, StatusCode::BAD_REQUEST)
}

pub fn json_error_message_status(message: &str, status: StatusCode) -> HttpResponse {
    json_error(
        JsonResponseEmptyMessage {},
        status,
        Some(message.to_string()),
    )
}

pub fn json_success_message(message: &str) -> HttpResponse {
    json_success(JsonResponseEmptyMessage {}, Some(message.to_string()))
}

#[allow(dead_code)]
pub fn json_unauthorized_message(message: &str) -> HttpResponse {
    json_error_message_status(message, StatusCode::UNAUTHORIZED)
}

pub fn json_not_found_response(message: Option<&str>) -> HttpResponse {
    json_error_message_status(message.unwrap_or("Not Found"), StatusCode::NOT_FOUND)
}

pub fn json_entity_not_found_response(entity: &str) -> HttpResponse {
    json_not_found_response(Some(format!("Such {} does not exists", entity).as_str()))
}
