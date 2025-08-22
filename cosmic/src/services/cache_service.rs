use std::future::Future;

use crate::enums::app_message::AppMessage;
use log::debug;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::results::redis_result::RedisResultToAppResult;
use crate::results::AppResult;
use crate::services::redis_service::RedisService;

#[derive(Clone)]
pub struct CacheService {
    redis: RedisService,
}

impl CacheService {
    pub fn new(r: RedisService) -> CacheService {
        CacheService { redis: r }
    }

    pub fn redis(&self) -> &RedisService {
        &self.redis
    }

    pub fn put<T>(&mut self, key: &str, value: T) -> AppResult<String>
    where
        T: Serialize,
    {
        self.redis.set(key.to_string(), value).into_app_result()
    }

    pub fn get<T: DeserializeOwned>(&mut self, key: &str) -> AppResult<Option<T>> {
        let data = self
            .redis
            .get::<Option<String>>(key.to_string())
            .into_app_result()?;

        match data {
            None => Ok(None),
            Some(data) => Ok(Some(
                serde_json::from_str::<T>(&data).map_err(AppMessage::SerdeError)?,
            )),
        }
    }

    pub fn delete(&mut self, key: &str) -> AppResult<String> {
        self.redis.delete(key.to_string()).into_app_result()
    }

    pub fn get_or_put<'v, Val, Fun>(&mut self, key: &str, setter: Fun) -> AppResult<Val>
    where
        Val: Serialize + Deserialize<'v> + Clone,
        Fun: FnOnce(&mut Self) -> AppResult<Val>,
    {
        let result = self
            .redis
            .get::<Option<String>>(key.to_string())
            .into_app_result();

        match result {
            Ok(option) => match option {
                None => {
                    debug!("'{}' is missing in cache, executing setter()...", key);
                    match setter(self) {
                        Ok(value) => match self.put(key, value.clone()) {
                            Ok(_) => Ok(value),
                            Err(err) => Err(err),
                        },
                        Err(err) => Err(err),
                    }
                }
                Some(data) => {
                    debug!("'{}' collected from cache :)", key);
                    Ok(serde_json::from_str::<Val>(data.to_owned().leak()).unwrap())
                }
            },
            Err(err) => Err(err),
        }
    }

    pub async fn get_or_put_async<'v, Val, Fun, Fut>(
        &mut self,
        key: &str,
        setter: Fun,
    ) -> AppResult<Val>
    where
        Val: Serialize + Deserialize<'v> + Clone,
        Fun: FnOnce(&mut Self) -> Fut + Send + 'static,
        Fut: Future<Output = AppResult<Val>> + Send + 'static,
    {
        let result = self
            .redis
            .get::<Option<String>>(key.to_string())
            .into_app_result();

        match result {
            Ok(option) => match option {
                None => {
                    debug!("'{}' is missing in cache, executing setter()...", key);
                    match setter(self).await {
                        Ok(value) => match self.put(key, value.clone()) {
                            Ok(_) => Ok(value),
                            Err(err) => Err(err),
                        },
                        Err(err) => Err(err),
                    }
                }
                Some(data) => {
                    debug!("'{}' collected from cache :)", key);
                    Ok(serde_json::from_str::<Val>(data.to_owned().leak()).unwrap())
                }
            },
            Err(err) => Err(err),
        }
    }
}
