use crate::enums::app_message::AppMessage;
use crate::helpers::DBPool;
use crate::models::ui_menu::{CreateForm, UiMenu};
use crate::models::ui_menu_item::CreateForm as MenuItemCreateForm;
use crate::repositories::ui_menu_repository::UiMenuRepository;
use crate::results::AppResult;
use crate::services::ui_menu_item_service::UiMenuItemService;
use uuid::Uuid;

pub struct UiMenuService;

impl UiMenuService {
    pub fn create(
        &mut self,
        pool: &DBPool,
        created_by: Uuid,
        form: CreateForm,
    ) -> AppResult<UiMenu> {
        let menu = UiMenuRepository.create(pool, created_by, form.clone())?;

        if !form.has_items {
            let result = UiMenuItemService.create(
                pool,
                created_by,
                MenuItemCreateForm {
                    menu_id: menu.ui_menu_id,
                    url: form.url.clone().unwrap(),
                    name: form.name.clone(),
                    desc: form.desc,
                    priority: 1,
                },
            );

            if result.is_err() {
                let _ = self
                    .delete(pool, menu.ui_menu_id)
                    .map_err(|e| AppMessage::DatabaseErrorMessage(e.to_string()))?;
            }
        }

        Ok(menu)
    }

    pub fn update(&mut self, pool: &DBPool, id: Uuid, form: CreateForm) -> AppResult<UiMenu> {
        UiMenuRepository.update(pool, id, form)
    }

    pub fn delete(&mut self, pool: &DBPool, id: Uuid) -> AppResult<UiMenu> {
        UiMenuRepository.delete(pool, id)
    }
}
