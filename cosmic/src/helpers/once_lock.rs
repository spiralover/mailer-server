use std::sync::OnceLock;

use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use log::error;
use r2d2::PooledConnection;

use crate::app_state::{AppRedisQueues, AppState};
use crate::helpers::DBPool;
use crate::services::cache_service::CacheService;
use crate::services::redis_next_service::RedisNextService;
use crate::services::redis_service::RedisService;
use crate::MAILER;

pub trait OnceLockHelper<'a> {
    fn app(&self) -> &'a AppState {
        MAILER.get().unwrap()
    }

    fn database(&self) -> &'a DBPool {
        MAILER.get().unwrap().database()
    }

    fn redis(&self) -> RedisService {
        MAILER.get().unwrap().services.redis.clone()
    }

    fn cache(&self) -> CacheService {
        MAILER.get().unwrap().services.cache.clone()
    }

    fn redis_next(&self) -> &RedisNextService {
        &MAILER.get().unwrap().services.redis_next
    }

    fn redis_queues(&self) -> &AppRedisQueues {
        &MAILER.get().unwrap().redis_queues
    }

    fn db_conn(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.database().get().unwrap_or_else(|err| {
            error!("database error: {:?}", err);
            panic!("Failed to acquire database connection from connection pools")
        })
    }
}

impl<'a> OnceLockHelper<'a> for OnceLock<AppState> {}
