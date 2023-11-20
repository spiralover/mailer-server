use std::ops::DerefMut;

use diesel::{
    BoolExpressionMethods, ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl,
    SaveChangesDsl,
};
use uuid::Uuid;

use crate::enums::app_message::AppMessage;
use crate::helpers::db::{current_timestamp, OptionalResult};
use crate::helpers::db_pagination::{Paginate, PaginationResult};
use crate::helpers::get_db_conn;
use crate::helpers::http::QueryParams;
use crate::models::ui_menu_item::{CreateForm, UiMenuItem};
use crate::models::DBPool;
use crate::results::app_result::FormatAppResult;
use crate::results::http_result::ErroneousOptionResponse;
use crate::results::AppResult;
use crate::schema::ui_menu_items;

pub struct UiMenuItemRepository;

impl UiMenuItemRepository {
    pub fn list_paginated(
        &mut self,
        pool: &DBPool,
        q: QueryParams,
    ) -> AppResult<PaginationResult<UiMenuItem>> {
        ui_menu_items::table
            .filter(ui_menu_items::deleted_at.is_null())
            .filter(
                ui_menu_items::mi_name
                    .ilike(q.get_search_query_like())
                    .or(ui_menu_items::mi_desc.ilike(q.get_search_query_like())),
            )
            .paginate(q.get_page())
            .per_page(q.get_per_page())
            .load_and_count_pages::<UiMenuItem>(get_db_conn(pool).deref_mut())
            .into_app_result()
    }

    pub fn list(&mut self, pool: &DBPool) -> AppResult<Vec<UiMenuItem>> {
        ui_menu_items::table
            .filter(ui_menu_items::deleted_at.is_null())
            .get_results::<UiMenuItem>(get_db_conn(pool).deref_mut())
            .into_app_result()
    }

    pub fn list_by_menu_id(
        &mut self,
        pool: &DBPool,
        menu_id: Uuid,
        q: QueryParams,
    ) -> AppResult<PaginationResult<UiMenuItem>> {
        ui_menu_items::table
            .filter(ui_menu_items::deleted_at.is_null())
            .filter(ui_menu_items::ui_menu_id.eq(menu_id))
            .filter(
                ui_menu_items::mi_name
                    .ilike(q.get_search_query_like())
                    .or(ui_menu_items::mi_desc.ilike(q.get_search_query_like())),
            )
            .order_by(ui_menu_items::mi_priority.asc())
            .paginate(q.get_page())
            .per_page(q.get_per_page())
            .load_and_count_pages::<UiMenuItem>(get_db_conn(pool).deref_mut())
            .into_app_result()
    }

    pub fn create(
        &mut self,
        pool: &DBPool,
        created_by: Uuid,
        form: CreateForm,
    ) -> AppResult<UiMenuItem> {
        let model = UiMenuItem {
            ui_menu_item_id: Uuid::new_v4(),
            created_by,
            ui_menu_id: form.menu_id,
            mi_name: form.name,
            mi_priority: form.priority,
            mi_desc: form.desc,
            mi_url: form.url,
            created_at: current_timestamp(),
            updated_at: current_timestamp(),
            deleted_at: None,
        };

        diesel::insert_into(ui_menu_items::dsl::ui_menu_items)
            .values(model)
            .get_result::<UiMenuItem>(get_db_conn(pool).deref_mut())
            .into_app_result()
    }

    pub fn update(&mut self, pool: &DBPool, id: Uuid, form: CreateForm) -> AppResult<UiMenuItem> {
        let result = self.find_by_id(pool, id);

        if result.is_error_or_empty() {
            return result.get_error_result();
        }

        diesel::update(
            ui_menu_items::dsl::ui_menu_items.filter(ui_menu_items::ui_menu_item_id.eq(id)),
        )
        .set((
            ui_menu_items::mi_name.eq(form.name),
            ui_menu_items::mi_priority.eq(form.priority),
            ui_menu_items::mi_desc.eq(form.desc),
            ui_menu_items::mi_url.eq(form.url),
            ui_menu_items::ui_menu_id.eq(form.menu_id),
            ui_menu_items::updated_at.eq(current_timestamp()),
        ))
        .execute(get_db_conn(pool).deref_mut())
        .into_app_result()?;

        Ok(self.find_by_id(pool, id).unwrap())
    }

    pub fn delete(&mut self, pool: &DBPool, id: Uuid) -> AppResult<UiMenuItem> {
        let result = self.find_by_id(pool, id);

        if result.is_error_or_empty() {
            return result.get_error_result();
        }

        diesel::update(
            ui_menu_items::dsl::ui_menu_items.filter(ui_menu_items::ui_menu_item_id.eq(id)),
        )
        .set(ui_menu_items::deleted_at.eq(current_timestamp()))
        .execute(get_db_conn(pool).deref_mut())
        .expect("Failed to delete ui menu item");

        let mut ui_menu_item = result.unwrap();
        ui_menu_item.deleted_at = Some(current_timestamp());
        ui_menu_item
            .save_changes::<UiMenuItem>(get_db_conn(pool).deref_mut())
            .map_err(|e| AppMessage::DatabaseError(e.to_string()))
    }

    pub fn find_by_id(&mut self, pool: &DBPool, id: Uuid) -> AppResult<UiMenuItem> {
        ui_menu_items::table
            .filter(ui_menu_items::ui_menu_item_id.eq(id))
            .filter(ui_menu_items::deleted_at.is_null())
            .first::<UiMenuItem>(get_db_conn(pool).deref_mut())
            .required("ui menu item")
    }
}
