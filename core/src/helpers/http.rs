use std::str::FromStr;

use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::MultipartForm;
use actix_web::http::header;
use actix_web::HttpRequest;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::enums::app_message::AppMessage;
use crate::helpers::string::string;

#[derive(Deserialize, Clone)]
pub struct QueryParams {
    pub search: Option<String>,
    pub status: Option<String>,
    pub stage: Option<String>,
    pub limit: Option<i64>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(MultipartForm)]
pub struct UploadForm {
    pub file: TempFile,
}

#[derive(Deserialize)]
pub struct IdPathParam {
    pub id: String,
}

#[derive(Deserialize)]
pub struct IdAsUuid {
    pub id: Uuid,
}

#[derive(Deserialize, Validate)]
pub struct ReasonPayload {
    #[validate(length(min = 3, max = 1500))]
    pub reason: String,
}

impl IdPathParam {
    #[allow(dead_code)]
    pub fn get_uuid(&mut self) -> Result<Uuid, AppMessage> {
        Uuid::from_str(self.id.clone().as_str()).map_err(|_e| AppMessage::InvalidUUID)
    }
}

impl QueryParams {
    pub fn get_search_query(&self) -> String {
        self.search.clone().unwrap_or(string(""))
    }

    pub fn get_search_query_like(&self) -> String {
        format!("%{}%", self.get_search_query())
    }

    #[allow(dead_code)]
    pub fn get_limit(&self) -> i64 {
        self.limit.unwrap_or(10)
    }

    pub fn get_page(&self) -> i64 {
        self.page.unwrap_or(1)
    }

    pub fn get_per_page(&self) -> i64 {
        self.per_page.unwrap_or(10)
    }
}

pub fn get_ip_and_ua(req: HttpRequest) -> (Option<String>, Option<String>) {
    let user_agent = req
        .headers()
        .get(header::USER_AGENT)
        .map(|u| u.to_str().unwrap().to_string());

    let ip_address = req
        .connection_info()
        .realip_remote_addr()
        .map(|v| v.to_string())
        .unwrap_or(req.peer_addr().map(|s| s.to_string()).unwrap());

    (Some(ip_address), user_agent)
}

#[allow(dead_code)]
pub fn date_from_unsafe_input(date: &str, field_name: &str) -> Result<NaiveDateTime, AppMessage> {
    NaiveDateTime::parse_from_str(format!("{} 00:00:00", date).as_str(), "%Y-%m-%d %H:%M:%S")
        .map_err(|e| {
            let msg = Box::leak(Box::new(format!(
                "Invalid {} input value({}), please make sure it's valid date; {}",
                field_name, date, e
            )));

            AppMessage::WarningMessage(msg)
        })
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HttpHeaderItem {
    pub name: String,
    pub value: String,
}
