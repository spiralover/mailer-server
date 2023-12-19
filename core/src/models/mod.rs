use diesel::{r2d2::ConnectionManager, PgConnection};

pub mod announcement;
pub mod app_key;
pub mod application;
pub mod auth_attempt;
pub mod file_upload;
pub mod notification;
pub mod password_reset;
pub mod permission;
pub mod role;
pub mod role_permission;
pub mod ui_menu;
pub mod ui_menu_item;
pub mod user;
pub mod user_permission;
pub mod user_role;
pub mod user_ui_menu_item;
pub mod mail;
pub mod mail_address;
pub mod mail_error;

// type alias to use in multiple places
pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub trait Model {}
