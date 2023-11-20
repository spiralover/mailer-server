use actix_web::HttpResponse;
use diesel::result::Error;
use diesel::QueryResult;
use serde::Serialize;

use crate::enums::app_message::AppMessage;
use crate::helpers::db_pagination::PaginationResult;
use crate::helpers::responder::{json_pagination, json_success};
use crate::results::http_result::{ErroneousOptionResponse, ErroneousResponse, PaginationResponse};
use crate::results::{AppResult, HttpResult};

pub type AppOptionalResult<T> = Result<Option<T>, AppMessage>;

pub trait FormatAppResult<T> {
    fn into_app_result(self) -> AppResult<T>;
}

impl<T: Serialize> ErroneousOptionResponse<T> for AppResult<T> {
    fn is_empty(&self) -> bool {
        if let Err(AppMessage::EntityNotFound(..)) = self {
            return true;
        }

        false
    }

    fn is_error_or_empty(&self) -> bool {
        self.as_ref().is_err() || self.is_empty()
    }

    fn get_error_result(self) -> AppResult<T> {
        if self.is_err() {
            return Err(self.err().unwrap());
        }

        // let entity = self.
        panic!("Cannot acquire error on successful database action")
    }

    fn send_error(self) -> HttpResult {
        if self.is_err() {
            return Err(self.err().unwrap());
        }

        Err(AppMessage::WarningMessage("Internal Server Error"))
    }

    fn send_entity(self) -> HttpResult {
        Ok(json_success(self.unwrap(), None))
    }

    fn send_response(self) -> HttpResult {
        if self.is_error_or_empty() {
            return self.send_error();
        }

        self.send_entity()
    }
}

impl<T: Serialize> ErroneousResponse for AppResult<T> {
    fn send_result(self) -> HttpResult {
        if self.is_err() {
            return Err(self.err().unwrap());
        }

        Ok(json_success(self.unwrap(), None))
    }
}

impl<T: Serialize> PaginationResponse for AppResult<PaginationResult<T>> {
    fn send_pagination(self) -> HttpResponse {
        json_pagination(self.unwrap())
    }

    fn send_pagination_result(self) -> HttpResult {
        let data = self?;
        Ok(json_pagination(data))
    }
}

impl<T> FormatAppResult<T> for QueryResult<T> {
    fn into_app_result(self) -> AppResult<T> {
        match self {
            Ok(value) => Ok(value),
            Err(Error::NotFound) => Err(AppMessage::DatabaseEntityNotFound),
            Err(e) => Err(AppMessage::DatabaseError(e.to_string())),
        }
    }
}
