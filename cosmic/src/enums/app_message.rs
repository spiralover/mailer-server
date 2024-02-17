use std::ffi::NulError;
use std::fmt::{Debug, Display, Formatter};
use std::io;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use diesel::result::{DatabaseErrorInformation, DatabaseErrorKind, Error};
use log::error;
use serde::de::StdError;
use validator::ValidationErrors;

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
    RedisPoolError(mobc::Error<redis::RedisError>),
    SerdeError(serde_json::Error),
    ReqwestError(reqwest::Error),
    FromUtf8Error(std::string::FromUtf8Error),
    MailerError(reqwest::Error),
    NerveError(reqwest::Error),
    EntityNotFound(String),
    WarningMessage(String),
    WarningMessageStr(&'static str),
    SuccessMessage(String),
    SuccessMessageStr(&'static str),
    ErrorMessage(String, StatusCode),
    UnAuthorizedMessage(&'static str),
    FormValidationError(ValidationErrors),
    BlockingError(actix_web::error::BlockingError),
    JoinError(tokio::task::JoinError),

    DatabaseError(
        DatabaseErrorKind,
        Box<dyn DatabaseErrorInformation + Send + Sync>,
    ),
    DatabaseRollbackErrorOnCommit {
        rollback_error: Box<Error>,
        commit_error: Box<Error>,
    },
    DatabaseErrorMessage(String),
    DatabaseEntityNotFound,
    DatabaseInvalidCString(NulError),
    DatabaseQueryBuilderError(Box<dyn StdError + Send + Sync>),
    DatabaseDeserializationError(Box<dyn StdError + Send + Sync>),
    DatabaseSerializationError(Box<dyn StdError + Send + Sync>),
    DatabaseRollbackTransaction,
    DatabaseAlreadyInTransaction,
    DatabaseNotInTransaction,
    DatabaseBrokenTransactionManager,
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

impl From<io::Error> for AppMessage {
    fn from(value: io::Error) -> Self {
        AppMessage::IoError(value)
    }
}

impl From<actix_web::error::BlockingError> for AppMessage {
    fn from(value: actix_web::error::BlockingError) -> Self {
        AppMessage::BlockingError(value)
    }
}

impl From<tokio::task::JoinError> for AppMessage {
    fn from(value: tokio::task::JoinError) -> Self {
        AppMessage::JoinError(value)
    }
}

impl From<redis::RedisError> for AppMessage {
    fn from(value: redis::RedisError) -> Self {
        AppMessage::RedisError(value)
    }
}

impl From<serde_json::Error> for AppMessage {
    fn from(value: serde_json::Error) -> Self {
        AppMessage::SerdeError(value)
    }
}

impl From<reqwest::Error> for AppMessage {
    fn from(value: reqwest::Error) -> Self {
        AppMessage::ReqwestError(value)
    }
}

impl From<std::string::FromUtf8Error> for AppMessage {
    fn from(value: std::string::FromUtf8Error) -> Self {
        AppMessage::FromUtf8Error(value)
    }
}

impl AppMessage {
    pub fn ok(&self) -> HttpResult {
        Ok(send_response(self))
    }

    pub fn into_response(self) -> HttpResponse {
        send_response(&self)
    }
}

impl From<Error> for AppMessage {
    fn from(value: Error) -> Self {
        match value {
            Error::InvalidCString(err) => AppMessage::DatabaseErrorMessage(err.to_string()),
            Error::DatabaseError(x, y) => AppMessage::DatabaseError(x, y),
            Error::NotFound => AppMessage::DatabaseEntityNotFound,
            Error::QueryBuilderError(err) => AppMessage::DatabaseQueryBuilderError(err),
            Error::DeserializationError(err) => AppMessage::DatabaseDeserializationError(err),
            Error::SerializationError(err) => AppMessage::DatabaseDeserializationError(err),
            Error::RollbackErrorOnCommit {
                commit_error,
                rollback_error,
            } => AppMessage::DatabaseRollbackErrorOnCommit {
                commit_error,
                rollback_error,
            },
            Error::RollbackTransaction => AppMessage::DatabaseRollbackTransaction,
            Error::AlreadyInTransaction => AppMessage::DatabaseAlreadyInTransaction,
            Error::NotInTransaction => AppMessage::DatabaseNotInTransaction,
            Error::BrokenTransactionManager => AppMessage::DatabaseBrokenTransactionManager,
            _ => AppMessage::InternalServerError,
        }
    }
}

impl From<AppMessage> for Error {
    fn from(value: AppMessage) -> Self {
        match value {
            AppMessage::EntityNotFound(_) => Error::NotFound,
            AppMessage::WarningMessageStr(err) => Error::QueryBuilderError(Box::from(err)),
            AppMessage::ErrorMessage(err, _) => Error::QueryBuilderError(Box::from(err)),
            AppMessage::DatabaseRollbackErrorOnCommit {
                rollback_error,
                commit_error,
            } => Error::RollbackErrorOnCommit {
                commit_error,
                rollback_error,
            },
            AppMessage::DatabaseErrorMessage(err) => Error::QueryBuilderError(Box::from(err)),
            AppMessage::DatabaseEntityNotFound => Error::NotFound,
            AppMessage::DatabaseInvalidCString(err) => Error::InvalidCString(err),
            AppMessage::DatabaseQueryBuilderError(err) => Error::QueryBuilderError(err),
            AppMessage::DatabaseDeserializationError(err) => Error::DeserializationError(err),
            AppMessage::DatabaseSerializationError(err) => Error::SerializationError(err),
            AppMessage::DatabaseRollbackTransaction => Error::RollbackTransaction,
            AppMessage::DatabaseAlreadyInTransaction => Error::AlreadyInTransaction,
            AppMessage::DatabaseNotInTransaction => Error::NotInTransaction,
            AppMessage::DatabaseBrokenTransactionManager => Error::BrokenTransactionManager,
            _ => Error::NotFound,
        }
    }
}

impl ResponseError for AppMessage {
    fn status_code(&self) -> StatusCode {
        match self {
            AppMessage::InvalidUUID => StatusCode::BAD_REQUEST,
            AppMessage::SuccessMessage(_msg) => StatusCode::OK,
            AppMessage::SuccessMessageStr(_msg) => StatusCode::OK,
            AppMessage::WarningMessage(_msg) => StatusCode::BAD_REQUEST,
            AppMessage::WarningMessageStr(_msg) => StatusCode::BAD_REQUEST,
            AppMessage::EntityNotFound(_msg) => StatusCode::NOT_FOUND,
            AppMessage::DatabaseEntityNotFound => StatusCode::NOT_FOUND,
            AppMessage::IoError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            AppMessage::SerdeError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            AppMessage::ReqwestError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            AppMessage::RedisError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            AppMessage::MailerError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            AppMessage::NerveError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            AppMessage::BlockingError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            AppMessage::ErrorMessage(_, status) => *status,
            AppMessage::UnAuthorized => StatusCode::UNAUTHORIZED,
            AppMessage::UnAuthorizedMessage(_) => StatusCode::UNAUTHORIZED,
            AppMessage::FormValidationError(_) => StatusCode::BAD_REQUEST,
            AppMessage::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR, // all database-related errors are 500
        }
    }

    fn error_response(&self) -> HttpResponse {
        let mut resp = send_response(self);
        resp.headers_mut()
            .insert("X-App-Response".parse().unwrap(), "True".parse().unwrap());
        resp
    }
}

fn format_message(status: &AppMessage, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(get_message(status).as_str())
}

fn get_message(status: &AppMessage) -> String {
    match status {
        AppMessage::InvalidUUID => String::from("Invalid unique identifier"),
        AppMessage::UnAuthorized => {
            string("You are not authorized to access requested resource(s)")
        }
        AppMessage::EntityNotFound(entity) => format!("Such {} does not exits", entity),
        AppMessage::DatabaseEntityNotFound => string("Such entity does not exits"),
        AppMessage::IoError(error) => error.to_string(),
        AppMessage::SerdeError(error) => error.to_string(),
        AppMessage::RedisError(error) => error.to_string(),
        AppMessage::FromUtf8Error(error) => error.to_string(),
        AppMessage::MailerError(error) => error.to_string(),
        AppMessage::NerveError(error) => error.to_string(),
        AppMessage::BlockingError(error) => error.to_string(),
        AppMessage::WarningMessage(message) => message.to_string(),
        AppMessage::WarningMessageStr(message) => message.to_string(),
        AppMessage::SuccessMessage(message) => message.to_string(),
        AppMessage::SuccessMessageStr(message) => message.to_string(),
        AppMessage::ErrorMessage(message, _) => message.clone(),
        AppMessage::UnAuthorizedMessage(message) => message.to_string(),
        AppMessage::FormValidationError(e) => String::from(e.to_string().as_str()),
        _ => String::from("Internal Server Error"),
    }
}

pub fn send_response(status: &AppMessage) -> HttpResponse {
    match status {
        AppMessage::EntityNotFound(entity) => json_entity_not_found_response(entity),
        AppMessage::IoError(message) => {
            error!("IO Error: {}", message);
            json_error_message_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
        }
        AppMessage::RedisError(message) => {
            error!("Redis Error: {}", message);
            json_error_message_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
        }
        AppMessage::RedisPoolError(message) => {
            error!("Redis Pool Error: {}", message);
            json_error_message_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
        }
        AppMessage::ReqwestError(message) => {
            error!("Http Client(Reqwest) Error: {}", message);
            json_error_message_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
        }
        AppMessage::FromUtf8Error(message) => {
            error!("Utf8 Conversion Error: {:?}", message);
            json_error_message_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
        }
        AppMessage::DatabaseErrorMessage(message) => {
            error!("DB Error: {}", message);
            json_error_message_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
        }
        AppMessage::BlockingError(message) => {
            error!("Blocking Error: {}", message);
            json_error_message_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
        }
        AppMessage::SuccessMessage(message) => json_success_message(message),
        AppMessage::SuccessMessageStr(message) => json_success_message(message),
        AppMessage::ErrorMessage(message, status) => json_error_message_status(message, *status),
        AppMessage::UnAuthorizedMessage(message) => {
            json_error_message_status(message, StatusCode::UNAUTHORIZED)
        }
        AppMessage::FormValidationError(e) => {
            json_error(e, StatusCode::BAD_REQUEST, Some(string("Validation Error")))
        }
        _ => json_error_message(get_message(status).as_str()),
    }
}
