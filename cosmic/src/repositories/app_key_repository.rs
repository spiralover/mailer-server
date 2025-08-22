use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use jsonwebtoken::{Algorithm, Header};
use uuid::Uuid;

use crate::helpers::db::{DatabaseConnectionHelper, OptionalResult};
use crate::helpers::security::generate_token;
use crate::helpers::time::current_timestamp;
use crate::helpers::DBPool;
use crate::models::app_key::AppKey;
use crate::results::app_result::FormatAppResult;
use crate::results::AppResult;
use crate::schema::app_keys;

pub struct AppKeyRepository;

impl AppKeyRepository {
    pub fn generate(&mut self, pool: &DBPool, app_id: Uuid, created_by: Uuid) -> AppResult<AppKey> {
        let public_key = generate_token(app_id.to_string(), None, None);
        let header = Header::new(Algorithm::HS512);
        let private_key = generate_token(app_id.to_string(), Some(header), None);
        let model = AppKey {
            app_key_id: Uuid::new_v4(),
            application_id: app_id,
            public_key: public_key.access_token,
            private_key: private_key.access_token,
            created_by,
            status: "active".to_string(),
            created_at: current_timestamp(),
            updated_at: current_timestamp(),
            deleted_at: None,
        };

        diesel::insert_into(app_keys::dsl::app_keys)
            .values(model)
            .get_result::<AppKey>(&mut pool.conn())
            .into_app_result()
    }

    pub fn find_by_id(&mut self, pool: &DBPool, id: Uuid) -> AppResult<AppKey> {
        app_keys::table
            .filter(app_keys::app_key_id.eq(id))
            .first::<AppKey>(&mut pool.conn())
            .required("application key")
    }

    pub fn find_active_by_app_id(&mut self, pool: &DBPool, id: Uuid) -> AppResult<AppKey> {
        app_keys::table
            .filter(app_keys::application_id.eq(id))
            .order_by(app_keys::created_by.desc())
            .filter(app_keys::status.eq(String::from("active")))
            .first::<AppKey>(&mut pool.conn())
            .required("application key")
    }
}
