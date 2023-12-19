use std::fmt::{Debug, Display, Formatter};
use std::io;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use log::error;
use serde::Serialize;
use validator::ValidationErrors;

use crate::enums::app_message::AppMessage::{
    BlockingError, DatabaseEntityNotFound, DatabaseError, EntityNotFound, ErrorMessage,
    InvalidUUID, IoError, RedisError, SuccessMessage, WarningMessage,
};
use crate::helpers::responder::{
    json_entity_not_found_response, json_error, json_error_message, json_error_message_status,
    json_success_message,
};
use crate::helpers::string::string;
use crate::results::HttpResult;

pub enum AppMessage {
    InvalidUUID,
    UnAuthorized,
    #[allow(dead_code)]
    InternalServerError,
    IoError(io::Error),
    RedisError(redis::RedisError),
    DatabaseError(String),
    EntityNotFound(String),
    DatabaseEntityNotFound,
    WarningMessage(&'static str),
    SuccessMessage(&'static str),
    ErrorMessage(String, StatusCode),
    FormValidationError(ValidationErrors),
    BlockingError(actix_web::error::BlockingError),
}

#[derive(Serialize)]
struct DefaultContext {}

impl AppMessage {
    pub fn ok(&self) -> HttpResult {
        Ok(send_response(self))
    }

    pub fn into_response(self) -> HttpResponse {
        send_response(&self)
    }
}

impl From<io::Error> for AppMessage {
    fn from(value: io::Error) -> Self {
        IoError(value)
    }
}

pub fn format_message(status: &AppMessage, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(get_message(status).as_str())
}

fn get_message(status: &AppMessage) -> String {
    match status {
        InvalidUUID => String::from("Invalid unique identifier"),
        AppMessage::InternalServerError => String::from("Internal Server Error"),
        AppMessage::UnAuthorized => {
            string("You are not authorized to access requested resource(s)")
        }
        EntityNotFound(entity) => format!("Such {} does not exits", entity),
        DatabaseEntityNotFound => string("Such entity does not exits"),
        IoError(error) => error.to_string(),
        RedisError(error) => error.to_string(),
        BlockingError(error) => error.to_string(),
        DatabaseError(message) => message.clone(),
        WarningMessage(message) => message.to_string(),
        SuccessMessage(message) => message.to_string(),
        ErrorMessage(message, _) => message.clone(),
        AppMessage::FormValidationError(e) => String::from(e.to_string().as_str()),
    }
}

pub fn send_response(status: &AppMessage) -> HttpResponse {
    match status {
        EntityNotFound(entity) => json_entity_not_found_response(entity),
        IoError(message) => {
            error!("IO Error: {}", message);
            json_error_message_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
        }
        RedisError(message) => {
            error!("Redis Error: {}", message);
            json_error_message_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
        }
        DatabaseError(message) => {
            error!("DB Error: {}", message);
            json_error_message_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
        }
        BlockingError(message) => {
            error!("Blocking Error: {}", message);
            json_error_message_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
        }
        ErrorMessage(message, status) => json_error_message_status(message, *status),
        SuccessMessage(message) => json_success_message(message),
        AppMessage::FormValidationError(e) => {
            json_error(e, StatusCode::BAD_REQUEST, Some(string("Validation Error")))
        }
        _ => json_error_message(get_message(status).as_str()),
    }
}

impl Debug for AppMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format_message(self, f)
    }
}

impl Display for AppMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format_message(self, f)
    }
}

impl From<ValidationErrors> for AppMessage {
    fn from(value: ValidationErrors) -> Self {
        AppMessage::FormValidationError(value)
    }
}

impl ResponseError for AppMessage {
    fn status_code(&self) -> StatusCode {
        match self {
            InvalidUUID => StatusCode::BAD_REQUEST,
            SuccessMessage(_msg) => StatusCode::OK,
            EntityNotFound(_msg) => StatusCode::NOT_FOUND,
            DatabaseEntityNotFound => StatusCode::NOT_FOUND,
            WarningMessage(_msg) => StatusCode::BAD_REQUEST,
            IoError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            RedisError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            DatabaseError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            BlockingError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorMessage(_, status) => *status,
            AppMessage::UnAuthorized => StatusCode::UNAUTHORIZED,
            AppMessage::FormValidationError(_) => StatusCode::BAD_REQUEST,
            AppMessage::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let mut resp = send_response(self);
        resp.headers_mut()
            .insert("X-App-Response".parse().unwrap(), "True".parse().unwrap());
        resp
    }
}
