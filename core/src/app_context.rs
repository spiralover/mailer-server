use crate::app_state::AppState;
use crate::enums::auth_permission::AuthPermission;
use crate::helpers::auth::verify_auth_permission;
use crate::helpers::DBPool;
use crate::models::user::User;
use crate::results::AppResult;
use std::sync::Arc;
use uuid::Uuid;

pub struct AppContext {
    pub(crate) app: Arc<AppState>,
    pub(crate) auth_id: Uuid,
    pub(crate) auth_user: User,
}

impl AppContext {
    pub fn database(&self) -> &DBPool {
        self.app.database()
    }

    pub fn app(&self) -> Arc<AppState> {
        self.app.to_owned()
    }

    pub fn auth_id(&self) -> Uuid {
        self.auth_id
    }

    pub fn auth_user(&self) -> User {
        self.auth_user.clone()
    }

    pub fn verify_user_permission(&self, p: AuthPermission) -> AppResult<()> {
        verify_auth_permission(self.database(), self.auth_id(), p)
    }
}
