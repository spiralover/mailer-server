use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JsonResponse<T: Serialize> {
    success: bool,
    data: T,
    status: u16,
}

#[derive(Serialize, Deserialize)]
pub struct JsonMessageResponse {
    message: String,
}

pub fn json<T: Serialize>(data: T, status: StatusCode) -> HttpResponse {
    HttpResponse::build(status).json(data)
}

pub fn json_success<T: Serialize>(data: T) -> HttpResponse {
    json(
        JsonResponse {
            success: true,
            status: 200,
            data,
        },
        StatusCode::OK,
    )
}

pub fn json_success_message(message: &str) -> HttpResponse {
    json_success(JsonMessageResponse {
        message: message.to_string(),
    })
}

pub fn json_error<T: Serialize>(data: T, status: StatusCode) -> HttpResponse {
    json(
        JsonResponse {
            success: false,
            status: status.as_u16(),
            data,
        },
        status,
    )
}

#[allow(dead_code)]
pub fn json_error_message(message: &str) -> HttpResponse {
    json_error_message_status(message, StatusCode::BAD_REQUEST)
}

#[allow(dead_code)]
pub fn json_error_message_status(message: &str, status: StatusCode) -> HttpResponse {
    json_error(
        JsonMessageResponse {
            message: message.to_string(),
        },
        status,
    )
}
