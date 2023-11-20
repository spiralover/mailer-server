use uuid::Uuid;

use crate::models::ui_menu_item::{CreateForm, UiMenuItem};
use crate::models::DBPool;
use crate::repositories::ui_menu_item_repository::UiMenuItemRepository;
use crate::results::AppResult;

pub struct UiMenuItemService;

impl UiMenuItemService {
    pub fn create(
        &mut self,
        pool: &DBPool,
        created_by: Uuid,
        form: CreateForm,
    ) -> AppResult<UiMenuItem> {
        UiMenuItemRepository.create(pool, created_by, form)
    }

    pub fn update(&mut self, pool: &DBPool, id: Uuid, form: CreateForm) -> AppResult<UiMenuItem> {
        UiMenuItemRepository.update(pool, id, form)
    }

    pub fn delete(&mut self, pool: &DBPool, id: Uuid) -> AppResult<UiMenuItem> {
        UiMenuItemRepository.delete(pool, id)
    }
}
