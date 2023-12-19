use actix_web::http::header;
use actix_web::web::Data;
use actix_web::{HttpMessage, HttpRequest};
use uuid::Uuid;

use crate::app_state::AppState;
use crate::enums::permissions::Permissions;
use crate::helpers::auth::check_permission;
use crate::helpers::DBPool;
use crate::models::user::User;
use crate::results::AppResult;

pub struct ClientInfo {
    pub ip: Option<String>,
    pub ua: Option<String>,
}

pub trait RequestHelper {
    fn auth_id(&self) -> Uuid;

    fn auth_user(&self) -> User;

    fn app_state(&self) -> &AppState;

    fn db_pool(&self) -> &DBPool;

    fn verify_user_permission(&self, p: Permissions) -> AppResult<()>;
    fn get_client_info(&self) -> ClientInfo;
}

impl RequestHelper for HttpRequest {
    fn auth_id(&self) -> Uuid {
        *self.extensions().get::<Uuid>().unwrap()
    }

    fn auth_user(&self) -> User {
        self.extensions().get::<User>().unwrap().clone()
    }

    fn app_state(&self) -> &AppState {
        self.app_data::<Data<AppState>>().unwrap()
    }

    fn db_pool(&self) -> &DBPool {
        self.app_state().database()
    }

    fn verify_user_permission(&self, p: Permissions) -> AppResult<()> {
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
