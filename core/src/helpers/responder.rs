use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::Serialize;

use crate::helpers::db_pagination::PageData;

#[derive(Serialize)]
pub struct JsonResponse<T: Serialize> {
    code: u16,
    success: bool,
    status: String,
    message: Option<String>,
    data: T,
}

#[derive(Serialize)]
pub struct PaginationResponse<T: Serialize> {
    pub(crate) success: bool,
    pub(crate) total_pages: i64,
    pub(crate) total_records: i64,
    pub(crate) status: u16,
    pub(crate) data: Vec<T>,
}

#[derive(Serialize)]
pub struct JsonMessageResponse {}

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
    json_error(JsonMessageResponse {}, status, Some(message.to_string()))
}

pub fn json_success_message(message: &str) -> HttpResponse {
    json_success(JsonMessageResponse {}, Some(message.to_string()))
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
