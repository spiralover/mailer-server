use std::future::Future;
use std::num::NonZeroUsize;
use std::time::Duration;

use actix_web::rt::time;
use futures_util::StreamExt;
use log::{error, info};
use mobc::Manager;
use mobc::{async_trait, Pool};
use redis::aio::Connection;
use redis::{Client, Msg};
use tokio::runtime::Handle;

use crate::helpers::once_lock::OnceLockHelper;
use crate::results::redis_result::RedisResultToAppResult;
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
    type Connection = Connection;
    type Error = redis::RedisError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let c = self.client.get_tokio_connection().await?;
        Ok(c)
    }

    async fn check(&self, mut conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        redis::cmd("PING").query_async(&mut conn).await?;
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

    pub async fn subscribe<F, Fut>(channel: String, func: F) -> AppResult<()>
    where
        F: FnOnce(AppResult<String>) -> Fut + Copy + Send + 'static,
        Fut: Future<Output = AppResult<()>> + Send + 'static,
    {
        let conn = MAILER.app().redis.get_tokio_connection().await?;

        info!("subscribing to: {}", channel.clone());

        let mut pubsub = conn.into_pubsub();
        pubsub.subscribe(&[channel.clone()]).await?;

        let mut stream = pubsub.into_on_message();
        while let Some(msg) = stream.next().await {
            let channel = channel.clone();
            Handle::current().spawn(async move {
                let msg: Msg = msg; // to make RustRover happy
                let received = msg.get_payload::<String>().into_app_result();

                match func(received).await {
                    Ok(_) => {}
                    Err(err) => {
                        error!(
                            "[channel-executor][{}] executor returned error: {:?}",
                            channel, err
                        );
                    }
                };
            });
        }

        Ok(())
    }
}
