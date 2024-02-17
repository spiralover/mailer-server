use diesel::dsl::not;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl,
    SaveChangesDsl,
};
use uuid::Uuid;

use crate::enums::app_message::AppMessage;
use crate::helpers::db::{DatabaseConnectionHelper, OptionalResult};
use crate::helpers::db_pagination::{PageData, Paginate};
use crate::helpers::http::QueryParams;
use crate::helpers::time::current_timestamp;
use crate::helpers::DBPool;
use crate::models::ui_menu::UiMenu;
use crate::models::ui_menu_item::UiMenuItem;
use crate::models::user_ui_menu_item::{MenuItemCreateDto, UserUiMenuItem};
use crate::results::app_result::FormatAppResult;
use crate::results::AppResult;
use crate::schema::{ui_menu_items, ui_menus, user_ui_menu_items};

pub struct UserUiMenuItemRepository;

impl UserUiMenuItemRepository {
    pub fn list_menu_by_user_id(
        &mut self,
        pool: &DBPool,
        user_id: Uuid,
        q: QueryParams,
    ) -> AppResult<PageData<UiMenu>> {
        let sub_query = user_ui_menu_items::table
            .select(user_ui_menu_items::ui_menu_id)
            .filter(user_ui_menu_items::deleted_at.is_null())
            .filter(user_ui_menu_items::user_id.eq(user_id));

        ui_menus::table
            .filter(ui_menus::deleted_at.is_null())
            .filter(ui_menus::ui_menu_id.eq_any(sub_query))
            .filter(
                ui_menus::m_name
                    .ilike(q.get_search_query_like())
                    .or(ui_menus::m_desc.ilike(q.get_search_query_like())),
            )
            .paginate(q.get_page())
            .per_page(q.get_per_page())
            .load_and_count_pages::<UiMenu>(&mut pool.conn())
            .into_app_result()
    }

    pub fn list_menu_item_by_user_id(
        &mut self,
        pool: &DBPool,
        user_id: Uuid,
        q: QueryParams,
    ) -> AppResult<PageData<UiMenuItem>> {
        let sub_query = user_ui_menu_items::table
            .select(user_ui_menu_items::ui_menu_item_id)
            .filter(user_ui_menu_items::deleted_at.is_null())
            .filter(user_ui_menu_items::user_id.eq(user_id));

        ui_menu_items::table
            .filter(ui_menu_items::deleted_at.is_null())
            .filter(ui_menu_items::ui_menu_item_id.eq_any(sub_query))
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

    pub fn create(
        &mut self,
        pool: &DBPool,
        created_by: Uuid,
        menu_id: Uuid,
        form: MenuItemCreateDto,
    ) -> AppResult<UserUiMenuItem> {
        let model = UserUiMenuItem {
            user_ui_menu_item_id: Uuid::new_v4(),
            created_by,
            ui_menu_id: menu_id,
            ui_menu_item_id: form.menu_item_id,
            user_id: form.user_id,
            created_at: current_timestamp(),
            updated_at: current_timestamp(),
            deleted_at: None,
        };

        diesel::insert_into(user_ui_menu_items::dsl::user_ui_menu_items)
            .values(model)
            .get_result::<UserUiMenuItem>(&mut pool.conn())
            .into_app_result()
    }

    pub fn delete_by_item_id(
        &mut self,
        pool: &DBPool,
        user_id: Uuid,
        id: Uuid,
    ) -> AppResult<UserUiMenuItem> {
        let mut ui_menu_item = self.find_by_item_id(pool, user_id, id)?;

        diesel::update(
            user_ui_menu_items::table.filter(user_ui_menu_items::ui_menu_item_id.eq(id)),
        )
        .set(user_ui_menu_items::deleted_at.eq(current_timestamp()))
        .execute(&mut pool.conn())
        .into_app_result()?;

        ui_menu_item.deleted_at = Some(current_timestamp());
        ui_menu_item
            .save_changes::<UserUiMenuItem>(&mut pool.conn())
            .map_err(|e| AppMessage::DatabaseErrorMessage(e.to_string()))
    }

    pub fn list_assignable(&mut self, pool: &DBPool, user_id: Uuid) -> AppResult<Vec<UiMenuItem>> {
        let sub_query = user_ui_menu_items::table
            .select(user_ui_menu_items::ui_menu_item_id)
            .filter(user_ui_menu_items::deleted_at.is_null())
            .filter(user_ui_menu_items::user_id.eq(user_id));

        ui_menu_items::table
            .filter(ui_menu_items::deleted_at.is_null())
            .filter(not(ui_menu_items::ui_menu_item_id.eq_any(sub_query)))
            .get_results::<UiMenuItem>(&mut pool.conn())
            .into_app_result()
    }

    pub fn find_by_item_id(
        &mut self,
        pool: &DBPool,
        user_id: Uuid,
        id: Uuid,
    ) -> AppResult<UserUiMenuItem> {
        user_ui_menu_items::table
            .filter(user_ui_menu_items::ui_menu_item_id.eq(id))
            .filter(user_ui_menu_items::user_id.eq(user_id))
            .filter(user_ui_menu_items::deleted_at.is_null())
            .first::<UserUiMenuItem>(&mut pool.conn())
            .required("user ui menu item")
    }

    #[allow(dead_code)]
    pub fn find_by_id(&mut self, pool: &DBPool, id: Uuid) -> AppResult<UserUiMenuItem> {
        user_ui_menu_items::table
            .filter(user_ui_menu_items::user_ui_menu_item_id.eq(id))
            .filter(user_ui_menu_items::deleted_at.is_null())
            .first::<UserUiMenuItem>(&mut pool.conn())
            .required("user ui menu item")
    }
}
