use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::PooledConnection;

pub mod auth;
pub mod db;
pub mod db_pagination;
pub mod form;
pub mod fs;
pub mod hmac;
pub mod http;
pub mod id_generator;
pub mod misc;
pub mod number;
pub mod once_lock;
pub mod request;
pub mod responder;
pub mod security;
pub mod string;
pub mod time;
pub mod uuid;
pub mod validator;

pub fn get_db_conn(pool: &DBPool) -> PooledConnection<ConnectionManager<PgConnection>> {
    pool.get()
        .unwrap_or_else(|_| panic!("Failed to acquire database connection from connection pools"))
}

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;
