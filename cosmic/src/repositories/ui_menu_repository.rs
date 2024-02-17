use diesel::{
    BoolExpressionMethods, ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl,
    SaveChangesDsl,
};
use uuid::Uuid;

use crate::helpers::db::{DatabaseConnectionHelper, OptionalResult};
use crate::helpers::db_pagination::{PageData, Paginate};
use crate::helpers::http::QueryParams;
use crate::helpers::time::current_timestamp;
use crate::helpers::DBPool;
use crate::models::ui_menu::{CreateForm, UiMenu};
use crate::results::app_result::FormatAppResult;
use crate::results::AppResult;
use crate::schema::ui_menus;

pub struct UiMenuRepository;

impl UiMenuRepository {
    pub fn list(&mut self, pool: &DBPool, q: QueryParams) -> AppResult<PageData<UiMenu>> {
        ui_menus::table
            .filter(ui_menus::deleted_at.is_null())
            .filter(
                ui_menus::m_name
                    .ilike(q.get_search_query_like())
                    .or(ui_menus::m_desc.ilike(q.get_search_query_like())),
            )
            .order_by(ui_menus::m_priority.asc())
            .paginate(q.get_page())
            .per_page(q.get_per_page())
            .load_and_count_pages::<UiMenu>(&mut pool.conn())
            .into_app_result()
    }

    pub fn create(
        &mut self,
        pool: &DBPool,
        created_by: Uuid,
        form: CreateForm,
    ) -> AppResult<UiMenu> {
        diesel::insert_into(ui_menus::dsl::ui_menus)
            .values(UiMenu {
                ui_menu_id: Uuid::new_v4(),
                created_by,
                m_name: form.name,
                m_priority: form.priority,
                m_desc: form.desc,
                m_url: form.url,
                m_has_items: form.has_items,
                created_at: current_timestamp(),
                updated_at: current_timestamp(),
                deleted_at: None,
            })
            .get_result::<UiMenu>(&mut pool.conn())
            .into_app_result()
    }

    pub fn update(&mut self, pool: &DBPool, id: Uuid, form: CreateForm) -> AppResult<UiMenu> {
        let mut ui_menu = self.find_by_id(pool, id)?;
        ui_menu.m_name = form.name;
        ui_menu.m_desc = form.desc;
        ui_menu.m_priority = form.priority;
        ui_menu
            .save_changes::<UiMenu>(&mut pool.conn())
            .into_app_result()
    }

    pub fn delete(&mut self, pool: &DBPool, id: Uuid) -> AppResult<UiMenu> {
        let mut ui_menu = self.find_by_id(pool, id)?;
        ui_menu.deleted_at = Some(current_timestamp());
        ui_menu
            .save_changes::<UiMenu>(&mut pool.conn())
            .into_app_result()
    }

    pub fn find_by_id(&mut self, pool: &DBPool, id: Uuid) -> AppResult<UiMenu> {
        ui_menus::table
            .filter(ui_menus::ui_menu_id.eq(id))
            .filter(ui_menus::deleted_at.is_null())
            .first::<UiMenu>(&mut pool.conn())
            .required("ui menu")
    }
}
