#![allow(clippy::extra_unused_lifetimes)]

use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::schema::ui_menus;

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Insertable,
    Queryable,
    Selectable,
    AsChangeset,
    Identifiable,
    Clone,
)]
#[diesel(table_name = ui_menus)]
#[diesel(primary_key(ui_menu_id))]
pub struct UiMenu {
    pub ui_menu_id: Uuid,
    pub created_by: Uuid,
    pub m_name: String,
    pub m_priority: i32,
    pub m_desc: Option<String>,
    pub m_url: Option<String>,
    pub m_has_items: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateForm {
    pub name: String,
    pub priority: i32,
    pub desc: Option<String>,
    pub url: Option<String>,
    pub has_items: bool,
}

#[derive(Deserialize)]
pub struct UiMenuParam {
    pub menu_id: Uuid,
}
