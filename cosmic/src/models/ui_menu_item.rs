#![allow(clippy::extra_unused_lifetimes)]

use crate::models;
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
use models::ui_menu::UiMenu;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::schema::ui_menu_items;

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Insertable,
    Queryable,
    Associations,
    Selectable,
    AsChangeset,
    Identifiable,
    Clone,
)]
#[diesel(table_name = ui_menu_items)]
#[diesel(belongs_to(UiMenu))]
#[diesel(primary_key(ui_menu_item_id))]
pub struct UiMenuItem {
    pub ui_menu_item_id: Uuid,
    pub created_by: Uuid,
    pub ui_menu_id: Uuid,
    pub mi_name: String,
    pub mi_priority: i32,
    pub mi_desc: Option<String>,
    pub mi_url: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateForm {
    pub menu_id: Uuid,
    pub name: String,
    pub priority: i32,
    pub desc: Option<String>,
    pub url: String,
}

#[derive(Deserialize)]
pub struct UiMenuItemParam {
    pub menu_item_id: Uuid,
}
