use std::str::FromStr;

use diesel::{BelongingToDsl, ExpressionMethods, GroupedBy, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::helpers::db::DatabaseConnectionHelper;
use crate::helpers::DBPool;
use crate::models::ui_menu::UiMenu;
use crate::models::ui_menu_item::UiMenuItem;
use crate::models::user_ui_menu_item::{MenuItemCreateDto, UserMenuWithItems, UserUiMenuItem};
use crate::repositories::ui_menu_item_repository::UiMenuItemRepository;
use crate::repositories::user_ui_menu_item_repository::UserUiMenuItemRepository;
use crate::results::app_result::FormatAppResult;
use crate::results::AppResult;
use crate::schema::{ui_menu_items, ui_menus, user_ui_menu_items};
use crate::user_setup::UserSetup;

pub struct UserUiMenuItemService;

impl UserUiMenuItemService {
    pub fn create(
        &mut self,
        pool: &DBPool,
        created_by: Uuid,
        form: MenuItemCreateDto,
    ) -> AppResult<UserUiMenuItem> {
        let menu_item = UiMenuItemRepository.find_by_id(pool, form.menu_item_id)?;
        UserUiMenuItemRepository.create(pool, created_by, menu_item.ui_menu_id, form)
    }

    pub fn delete_by_item_id(
        &mut self,
        pool: &DBPool,
        user_id: Uuid,
        id: Uuid,
    ) -> AppResult<UserUiMenuItem> {
        UserUiMenuItemRepository.delete_by_item_id(pool, user_id, id)
    }

    pub fn get_items_for_profile(
        &mut self,
        pool: &DBPool,
        user_id: Uuid,
    ) -> AppResult<Vec<UserMenuWithItems>> {
        let menu_sub_query = user_ui_menu_items::table
            .select(user_ui_menu_items::ui_menu_id)
            .filter(user_ui_menu_items::user_id.eq(user_id))
            .filter(user_ui_menu_items::deleted_at.is_null());

        let menu_item_sub_query = user_ui_menu_items::table
            .select(user_ui_menu_items::ui_menu_item_id)
            .filter(user_ui_menu_items::user_id.eq(user_id))
            .filter(user_ui_menu_items::deleted_at.is_null());

        let menus = ui_menus::table
            .filter(ui_menus::deleted_at.is_null())
            .filter(ui_menus::ui_menu_id.eq_any(menu_sub_query))
            .order_by(ui_menus::m_priority.asc())
            .get_results::<UiMenu>(&mut pool.conn())
            .into_app_result()?;

        let menu_items = UiMenuItem::belonging_to(&menus)
            .filter(ui_menu_items::deleted_at.is_null())
            .filter(ui_menu_items::ui_menu_item_id.eq_any(menu_item_sub_query))
            .order_by(ui_menu_items::mi_priority.asc())
            .get_results::<UiMenuItem>(&mut pool.conn())
            .into_app_result()?;

        let items = menu_items
            .grouped_by(&menus)
            .into_iter()
            .zip(menus)
            .map(|(items, menu)| UserMenuWithItems { menu, items })
            .collect::<Vec<UserMenuWithItems>>();

        Ok(items)
    }

    pub fn give_user_basic_items(
        &mut self,
        pool: &DBPool,
        user_id: Uuid,
    ) -> Vec<AppResult<UserUiMenuItem>> {
        let menu_items = UserSetup::new().menu_items;
        let mut results = vec![];

        for menu_item in menu_items {
            let res = self.create(
                pool,
                user_id,
                MenuItemCreateDto {
                    user_id,
                    menu_item_id: Uuid::from_str(menu_item.as_str()).unwrap(),
                },
            );

            results.push(res);
        }

        results
    }
}
