use std::future::Future;
use std::num::NonZeroUsize;
use std::time::Duration;

use actix_web::rt::time;
use log::error;
use mobc::Manager;
use mobc::{async_trait, Pool};
pub use redis;
pub use redis::aio::MultiplexedConnection;
use redis::Client;
use tokio::runtime::Handle;

use crate::helpers::once_lock::OnceLockHelper;
use crate::results::AppResult;
use crate::MAILER;

pub type RedisPool = Pool<RedisConnectionManager>;

pub struct Redis;

pub struct RedisConnectionManager {
    client: Client,
}

impl RedisConnectionManager {
    pub fn new(c: Client) -> Self {
        Self { client: c }
    }
}

#[async_trait]
impl Manager for RedisConnectionManager {
    type Connection = MultiplexedConnection;
    type Error = redis::RedisError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let c = self.client.get_multiplexed_tokio_connection().await?;
        Ok(c)
    }

    async fn check(&self, mut conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        // Fix: Explicitly specify the return type for the PING command
        let _: String = redis::cmd("PING").query_async(&mut conn).await?;
        Ok(conn)
    }
}

impl Redis {
    pub fn secs(ms: u64) -> Option<u64> {
        Some(ms * 1000)
    }

    ///
    ///
    /// # Arguments
    ///
    /// * `queue`: redis queue you are polling
    /// * `interval`: interval within which the queue will be polled in microsecond, default: 500ms
    /// * `len`: total number of items to be pulled per each poll, default: 1
    /// * `func`: async function to be executed for each poll
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub async fn poll_queue<F, Fut>(
        queue: String,
        interval: Option<u64>,
        len: Option<NonZeroUsize>,
        func: F,
    ) where
        F: FnOnce(String) -> Fut + Copy + Send + 'static,
        Fut: Future<Output = AppResult<()>> + Send + 'static,
    {
        let mut interval = time::interval(Duration::from_micros(interval.unwrap_or(500)));

        loop {
            let queue = queue.clone();
            let popped = MAILER.redis_next().rpop(&queue.clone(), len).await;
            match popped {
                Ok(Some(item)) => {
                    Handle::current().spawn(async move {
                        match func(item).await {
                            Ok(_) => {}
                            Err(err) => {
                                error!(
                                    "[queue-poll][{}] executor returned error: {:?}",
                                    queue, err
                                );
                            }
                        }
                    });
                }
                Ok(None) => {
                    interval.tick().await;
                }
                Err(err) => {
                    error!("failed to pop queue: {:?}", err);
                    interval.tick().await;
                }
            };
        }
    }
}
