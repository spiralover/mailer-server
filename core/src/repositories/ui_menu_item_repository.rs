use crate::helpers::db::{DatabaseConnectionHelper, OptionalResult};
use crate::helpers::db_pagination::{PageData, Paginate};
use crate::helpers::http::QueryParams;
use crate::helpers::time::current_timestamp;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl,
    SaveChangesDsl,
};
use uuid::Uuid;

use crate::helpers::DBPool;
use crate::models::ui_menu_item::{CreateForm, UiMenuItem};
use crate::results::app_result::FormatAppResult;
use crate::results::AppResult;
use crate::schema::ui_menu_items;

pub struct UiMenuItemRepository;

impl UiMenuItemRepository {
    pub fn list_paginated(
        &mut self,
        pool: &DBPool,
        q: QueryParams,
    ) -> AppResult<PageData<UiMenuItem>> {
        ui_menu_items::table
            .filter(ui_menu_items::deleted_at.is_null())
            .filter(
                ui_menu_items::mi_name
                    .ilike(q.get_search_query_like())
                    .or(ui_menu_items::mi_desc.ilike(q.get_search_query_like())),
            )
            .paginate(q.get_page())
            .per_page(q.get_per_page())
            .load_and_count_pages::<UiMenuItem>(&mut pool.conn())
            .into_app_result()
    }

    pub fn list(&mut self, pool: &DBPool) -> AppResult<Vec<UiMenuItem>> {
        ui_menu_items::table
            .filter(ui_menu_items::deleted_at.is_null())
            .get_results::<UiMenuItem>(&mut pool.conn())
            .into_app_result()
    }

    pub fn list_by_menu_id(
        &mut self,
        pool: &DBPool,
        menu_id: Uuid,
        q: QueryParams,
    ) -> AppResult<PageData<UiMenuItem>> {
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
            .load_and_count_pages::<UiMenuItem>(&mut pool.conn())
            .into_app_result()
    }

    pub fn create(
        &mut self,
        pool: &DBPool,
        created_by: Uuid,
        form: CreateForm,
    ) -> AppResult<UiMenuItem> {
        diesel::insert_into(ui_menu_items::dsl::ui_menu_items)
            .values(UiMenuItem {
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
            })
            .get_result::<UiMenuItem>(&mut pool.conn())
            .into_app_result()
    }

    pub fn update(&mut self, pool: &DBPool, id: Uuid, form: CreateForm) -> AppResult<UiMenuItem> {
        let mut ui_menu_item = self.find_by_id(pool, id)?;
        ui_menu_item.mi_name = form.name;
        ui_menu_item.mi_priority = form.priority;
        ui_menu_item.mi_desc = form.desc;
        ui_menu_item.mi_url = form.url;
        ui_menu_item.ui_menu_id = form.menu_id;
        ui_menu_item
            .save_changes::<UiMenuItem>(&mut pool.conn())
            .into_app_result()
    }

    pub fn delete(&mut self, pool: &DBPool, id: Uuid) -> AppResult<UiMenuItem> {
        let mut ui_menu_item = self.find_by_id(pool, id)?;
        ui_menu_item.deleted_at = Some(current_timestamp());
        ui_menu_item
            .save_changes::<UiMenuItem>(&mut pool.conn())
            .into_app_result()
    }

    pub fn find_by_id(&mut self, pool: &DBPool, id: Uuid) -> AppResult<UiMenuItem> {
        ui_menu_items::table
            .filter(ui_menu_items::ui_menu_item_id.eq(id))
            .filter(ui_menu_items::deleted_at.is_null())
            .first::<UiMenuItem>(&mut pool.conn())
            .required("ui menu item")
    }
}
