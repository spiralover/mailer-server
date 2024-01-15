use diesel::r2d2::ConnectionManager;
use diesel::result::Error;
use diesel::{PgConnection, QueryResult};
use r2d2::PooledConnection;

use crate::enums::app_message::AppMessage;
use crate::enums::app_message::AppMessage::EntityNotFound;
use crate::helpers::DBPool;
use crate::results::app_result::AppOptionalResult;
use crate::results::AppResult;

pub trait OptionalResult<'a, T> {
    fn optional(self) -> AppOptionalResult<T>;
    fn required(self, entity: &'a str) -> AppResult<T>;
    fn exists(self) -> AppResult<bool>;
}

impl<'a, T> OptionalResult<'a, T> for QueryResult<T> {
    fn optional(self) -> AppResult<Option<T>> {
        match self {
            Ok(value) => Ok(Some(value)),
            Err(Error::NotFound) => Ok(None),
            Err(e) => Err(AppMessage::DatabaseErrorMessage(e.to_string())),
        }
    }

    fn required(self, entity: &'a str) -> AppResult<T> {
        match self {
            Ok(value) => Ok(value),
            Err(Error::NotFound) => Err(EntityNotFound(entity.to_string())),
            Err(e) => Err(AppMessage::DatabaseErrorMessage(e.to_string())),
        }
    }

    fn exists(self) -> AppResult<bool> {
        match self {
            Ok(_) => Ok(true),
            Err(Error::NotFound) => Ok(false),
            Err(e) => Err(AppMessage::DatabaseErrorMessage(e.to_string())),
        }
    }
}

pub trait DatabaseConnectionHelper {
    fn conn(&self) -> PooledConnection<ConnectionManager<PgConnection>>;
}

impl DatabaseConnectionHelper for DBPool {
    fn conn(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.get().unwrap_or_else(|_| {
            panic!("Failed to acquire database connection from connection pools")
        })
    }
}
