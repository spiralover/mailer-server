use actix_web::web::Data;
use actix_web::{HttpMessage, HttpRequest};
use uuid::Uuid;

use crate::app_state::AppState;
use crate::auth::check_permission;
use crate::models::user::User;
use crate::models::DBPool;
use crate::permissions::Permissions;
use crate::results::AppResult;

pub trait RequestHelper {
    fn auth_id(&self) -> Uuid;

    fn auth_user(&self) -> User;

    fn get_app_state(&self) -> &AppState;

    fn get_db_pool(&self) -> &DBPool;

    fn verify_user_permission(&self, p: Permissions) -> AppResult<()>;
}

impl RequestHelper for HttpRequest {
    fn auth_id(&self) -> Uuid {
        *self.extensions().get::<Uuid>().unwrap()
    }

    fn auth_user(&self) -> User {
        self.extensions().get::<User>().unwrap().clone()
    }

    fn get_app_state(&self) -> &AppState {
        self.app_data::<Data<AppState>>().unwrap()
    }

    fn get_db_pool(&self) -> &DBPool {
        self.get_app_state().get_db_pool()
    }

    fn verify_user_permission(&self, p: Permissions) -> AppResult<()> {
        check_permission(self.to_owned(), p)
    }
}
