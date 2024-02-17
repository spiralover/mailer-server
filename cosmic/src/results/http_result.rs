use actix_web::error::BlockingError;
use actix_web::HttpResponse;
use serde::Serialize;

use crate::enums::app_message::AppMessage;
use crate::helpers::db_pagination::PageData;
use crate::helpers::responder::{json_pagination, json_success};
use crate::models::Model;
use crate::results::app_result::ActixBlockResult;
use crate::results::{AppResult, HttpResult};

pub trait ErroneousResponse {
    fn send_result(self) -> HttpResult;
}

pub trait ActixBlockingResultResponder {
    fn respond(self) -> HttpResult;
}

pub trait PaginationResponse {
    fn send_pagination(self) -> HttpResponse;
    fn send_pagination_result(self) -> HttpResult;
}

pub trait StructResponse: Sized {
    fn send_response(self) -> HttpResponse;
    fn send_struct_result(self) -> Result<HttpResponse, AppMessage>;
}

pub trait ErroneousOptionResponse<T> {
    fn is_empty(&self) -> bool;
    fn is_error_or_empty(&self) -> bool;

    fn get_error_result(self) -> AppResult<T>;

    fn send_error(self) -> HttpResult;

    fn send_entity(self) -> HttpResult;

    fn send_response(self) -> HttpResult;
}

impl<T: Serialize> StructResponse for T {
    fn send_response(self) -> HttpResponse {
        json_success(self, None)
    }

    fn send_struct_result(self) -> HttpResult {
        Ok(self.send_response())
    }
}

impl ErroneousResponse for Result<AppMessage, AppMessage> {
    fn send_result(self) -> HttpResult {
        if self.is_err() {
            return Err(self.err().unwrap());
        }

        Ok(self.unwrap().into_response())
    }
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

        Err(AppMessage::WarningMessageStr("Internal Server Error"))
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

impl<T: Serialize> PaginationResponse for AppResult<PageData<T>> {
    fn send_pagination(self) -> HttpResponse {
        json_pagination(self.unwrap())
    }

    fn send_pagination_result(self) -> HttpResult {
        let data = self?;
        Ok(json_pagination(data))
    }
}

impl<T> ActixBlockingResultResponder for Result<T, BlockingError>
where
    T: Model + Serialize,
{
    fn respond(self) -> HttpResult {
        self.into_app_result().send_result()
    }
}

impl<T> ActixBlockingResultResponder for Result<AppResult<T>, BlockingError>
where
    T: Serialize + Sized,
{
    fn respond(self) -> HttpResult {
        <Result<T, AppMessage> as ErroneousResponse>::send_result(self.into_app_result())
    }
}

impl ActixBlockingResultResponder for Result<Result<AppMessage, AppMessage>, BlockingError> {
    fn respond(self) -> HttpResult {
        <Result<AppMessage, AppMessage> as ErroneousResponse>::send_result(self.into_app_result())
    }
}
