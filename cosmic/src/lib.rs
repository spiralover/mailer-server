use crate::app_state::AppState;
use std::sync::OnceLock;

pub mod app_context;
pub mod app_setup;
pub mod app_state;
pub mod enums;
pub mod helpers;
pub mod http;
pub mod models;
pub mod redis;
pub mod repositories;
pub mod results;
pub mod schema;
pub mod services;
pub mod user_setup;

pub static MAILER: OnceLock<AppState> = OnceLock::new();
