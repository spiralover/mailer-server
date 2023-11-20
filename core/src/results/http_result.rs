use actix_web::HttpResponse;
use serde::Serialize;

use crate::enums::app_message::AppMessage;
use crate::helpers::responder::json_success;
use crate::results::{AppResult, HttpResult};

pub trait ErroneousResponse {
    fn send_result(self) -> HttpResult;
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
