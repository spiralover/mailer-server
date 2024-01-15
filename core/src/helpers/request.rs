use std::sync::Arc;

use actix_web::http::header;
use actix_web::web::Data;
use actix_web::{HttpMessage, HttpRequest};
use uuid::Uuid;

use crate::app_context::AppContext;
use crate::app_state::AppState;
use crate::enums::auth_permission::AuthPermission;
use crate::models::user::UserCacheData;
use crate::results::AppResult;

use super::auth::check_permission;
use super::DBPool;

pub struct ClientInfo {
    pub ip: Option<String>,
    pub ua: Option<String>,
}

pub trait RequestHelper {
    fn auth_id(&self) -> Uuid;

    fn auth_user(&self) -> UserCacheData;

    fn app_state(&self) -> &AppState;

    fn context(&self) -> Arc<AppContext>;

    fn db_pool(&self) -> &DBPool;

    fn verify_user_permission(&self, p: AuthPermission) -> AppResult<()>;
    fn get_client_info(&self) -> ClientInfo;
}

impl RequestHelper for HttpRequest {
    fn auth_id(&self) -> Uuid {
        *self.extensions().get::<Uuid>().unwrap()
    }

    fn auth_user(&self) -> UserCacheData {
        self.extensions().get::<UserCacheData>().unwrap().clone()
    }

    fn app_state(&self) -> &AppState {
        self.app_data::<Data<AppState>>().unwrap()
    }

    fn context(&self) -> Arc<AppContext> {
        let app_state = self.app_data::<Data<AppState>>().unwrap();
        let auth_user = self.auth_user();
        Arc::new(AppContext {
            auth_id: auth_user.user_id,
            auth_user,
            app: app_state.clone().into_inner(),
        })
    }

    fn db_pool(&self) -> &DBPool {
        self.app_state().database()
    }

    fn verify_user_permission(&self, p: AuthPermission) -> AppResult<()> {
        check_permission(self.to_owned(), p)
    }

    fn get_client_info(&self) -> ClientInfo {
        let user_agent = self
            .headers()
            .get(header::USER_AGENT)
            .map(|u| u.to_str().unwrap().to_string());

        let ip_address = self
            .connection_info()
            .realip_remote_addr()
            .map(|v| v.to_string())
            .unwrap_or(self.peer_addr().map(|s| s.to_string()).unwrap());

        ClientInfo {
            ip: Some(ip_address),
            ua: user_agent,
        }
    }
}
