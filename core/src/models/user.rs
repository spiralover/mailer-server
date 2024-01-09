use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString, EnumVariantNames};
use uuid::Uuid;
use validator::Validate;

use crate::helpers::string::string;
use crate::models::permission::UserPermissionItem;
use crate::models::user_ui_menu_item::UserMenuWithItems;

use super::super::schema::users;

#[derive(
    Debug, Serialize, Deserialize, Insertable, Queryable, AsChangeset, Identifiable, Clone,
)]
#[diesel(table_name = users)]
#[diesel(primary_key(user_id))]
pub struct User {
    pub user_id: Uuid,
    pub created_by: Option<Uuid>,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub password: String,
    pub profile_picture: Option<String>,
    pub verification_code: Option<String>,
    pub verification_token: Option<String>,
    pub verified_at: Option<chrono::NaiveDateTime>,
    pub is_verified: bool,
    pub is_password_locked: bool,
    pub has_started_password_reset: bool,
    pub temp_password_status: String,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize)]
pub struct UserSharableData {
    pub user_id: Uuid,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub profile_picture: Option<String>,
    pub status: String,
    pub menu_items: Vec<UserMenuWithItems>,
    pub permissions: Vec<UserPermissionItem>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Queryable)]
pub struct UserMinimalData {
    #[diesel(sql_type = DieselUUID)]
    pub user_id: Uuid,
    #[diesel(sql_type = Text)]
    pub username: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub first_name: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub last_name: Option<String>,
    #[diesel(sql_type = Text)]
    pub email: String,
}

#[derive(Serialize, Queryable)]
pub struct UserVeryMinimalData {
    #[diesel(sql_type = DieselUUID)]
    pub user_id: Uuid,
    #[diesel(sql_type = Text)]
    pub username: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub first_name: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub last_name: Option<String>,
    #[diesel(sql_type = Text)]
    pub profile_picture: Option<String>,
}

#[derive(Serialize, Queryable, PartialEq)]
pub struct UserFullName {
    #[diesel(sql_type = DieselUUID)]
    pub user_id: Uuid,
    #[diesel(sql_type = Nullable<Text>)]
    pub first_name: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub last_name: Option<String>,
}

pub trait FullName {
    fn full_name(&self) -> String;
}

fn full_name(f: Option<String>, l: Option<String>) -> String {
    format!("{} {}", f.unwrap_or(string("")), l.unwrap_or(string("")))
        .trim()
        .to_string()
}

impl FullName for UserFullName {
    fn full_name(&self) -> String {
        full_name(self.first_name.clone(), self.last_name.clone())
    }
}

impl FullName for UserMinimalData {
    fn full_name(&self) -> String {
        full_name(self.first_name.clone(), self.last_name.clone())
    }
}

impl FullName for User {
    fn full_name(&self) -> String {
        full_name(self.first_name.clone(), self.last_name.clone())
    }
}

impl User {
    pub fn transform_response(&mut self) -> User {
        self.password = "".to_string();
        self.to_owned()
    }

    pub fn into_very_minimal_data(self) -> UserVeryMinimalData {
        UserVeryMinimalData {
            user_id: self.user_id,
            username: self.username,
            first_name: self.first_name,
            last_name: self.last_name,
            profile_picture: self.profile_picture,
        }
    }

    pub fn into_minimal_data(self) -> UserMinimalData {
        UserMinimalData {
            user_id: self.user_id,
            username: self.username,
            first_name: self.first_name,
            last_name: self.last_name,
            email: self.email,
        }
    }

    pub fn into_cache_data(self) -> UserCacheData {
        UserCacheData {
            user_id: self.user_id,
            username: self.username,
            email: self.email,
            roles: vec![],
        }
    }

    pub fn into_sharable(self) -> UserSharableData {
        UserSharableData {
            user_id: self.user_id,
            username: self.username,
            first_name: self.first_name,
            last_name: self.last_name,
            email: self.email,
            profile_picture: self.profile_picture,
            status: self.status,
            menu_items: vec![],
            permissions: vec![],
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserCacheData {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
}

#[derive(Clone, PartialEq, Display, Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum UserStatus {
    Active,
    Inactive,
    Pending,
}

#[derive(Clone, Display, Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum TempPasswordStatus {
    Used,
    UnUsed,
    Changed,
}

#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct AppCodeForm {
    pub code: String,
}

#[derive(Deserialize)]
pub struct AuthSsoToken {
    pub token: String,
}

#[derive(Serialize)]
pub struct AuthSsoData {
    pub token: String,
    pub callback: String,
}

#[derive(Deserialize)]
pub struct UsernameForm {
    pub username: String,
}

#[derive(Serialize)]
pub struct UsernameAvailability {
    pub is_available: bool,
    pub username: Option<String>,
    pub message: String,
}

#[derive(Deserialize, Validate)]
pub struct UserRegisterForm {
    pub created_by: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[validate(length(min = 4, max = 150))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 4, max = 50))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct UserUpdateForm {
    pub created_by: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[validate(length(min = 4, max = 150))]
    pub username: String,
    #[validate(email)]
    pub email: String,
}

#[derive(Serialize)]
pub struct ProfileData {
    user: User,
}

#[derive(Deserialize)]
pub struct EmailForm {
    pub email: String,
}

#[derive(Deserialize, Validate)]
pub struct PasswordForm {
    #[validate(must_match = "password_confirmation")]
    #[validate(length(min = 3, max = 50))]
    pub password: String,

    #[validate(length(min = 3, max = 50))]
    pub password_confirmation: String,
}
