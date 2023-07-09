use std::env;
use chrono::NaiveDateTime;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::{Pool, PooledConnection};

// type alias to use in multiple places
pub(crate) type DBPool = Pool<ConnectionManager<PgConnection>>;

pub(crate) fn create_database_connection() -> DBPool {
    let db_url: String = format!(
        "{}://{}:{}@{}:{}/{}",
        env::var("DB_DRIVER").unwrap(),
        env::var("DB_USERNAME").unwrap(),
        env::var("DB_PASSWORD").unwrap(),
        env::var("DB_HOST").unwrap(),
        env::var("DB_PORT").unwrap(),
        env::var("DB_DATABASE").unwrap(),
    );

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(db_url);
     Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub(crate) fn get_db_conn(pool: &DBPool) -> PooledConnection<ConnectionManager<PgConnection>> {
    pool.get()
        .unwrap_or_else(|_| panic!("Failed to acquire database connection from connection pools"))
}

pub(crate) fn current_timestamp() -> NaiveDateTime {
    chrono::Local::now().naive_local()
}
