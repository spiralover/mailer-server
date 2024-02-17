#![allow(clippy::extra_unused_lifetimes)]

use crate::models::ui_menu::UiMenu;
use crate::models::ui_menu_item::UiMenuItem;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::schema::user_ui_menu_items;

#[derive(
    Debug, Serialize, Deserialize, Insertable, Queryable, AsChangeset, Identifiable, Clone,
)]
#[diesel(table_name = user_ui_menu_items)]
#[diesel(primary_key(user_ui_menu_item_id))]
pub struct UserUiMenuItem {
    pub user_ui_menu_item_id: Uuid,
    pub created_by: Uuid,
    pub ui_menu_id: Uuid,
    pub ui_menu_item_id: Uuid,
    pub user_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct MenuItemCreateDto {
    pub menu_item_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Deserialize)]
pub struct UserUiMenuItemCreateForm {
    pub ids: Vec<Uuid>,
}

#[derive(Deserialize)]
pub struct UserUiMenuItemParam {
    pub menu_item_id: Uuid,
}

#[derive(Serialize)]
pub struct UserMenuWithItems {
    #[serde(flatten)]
    pub menu: UiMenu,
    pub items: Vec<UiMenuItem>,
}
