use crate::results::redis_result::ToLocalRedisResult;
use crate::results::RedisResult;
use log::{debug, error};
use redis::{Client, Commands};
use serde::Serialize;

#[derive(Clone)]
pub struct RedisService {
    redis: Client,
}

pub struct SubscribableQueue(pub String, pub String);

impl RedisService {
    pub fn new(redis: Client) -> RedisService {
        RedisService { redis }
    }

    pub fn push_to_queue<T: Serialize + Clone>(
        &mut self,
        queue: SubscribableQueue,
        data: T,
    ) -> RedisResult<i32> {
        let result = self.queue(queue.0, data.clone());

        // Push to respective channel
        debug!("[publisher]: publishing to {}", queue.1.clone());
        match self.publish(queue.1.clone(), data) {
            Ok(_) => {}
            Err(err) => error!("[publisher][{}]: {:?}", queue.1, err),
        };

        match result {
            Ok(result) => Ok(result),
            Err(error) => {
                error!("[queue][{}]: {:?}", queue.1, error);
                Err(error)
            }
        }
    }

    pub fn queue<T: Serialize>(&mut self, queue: String, data: T) -> redis::RedisResult<i32> {
        self.redis
            .lpush::<&str, &str, i32>(&*queue, serde_json::to_string(&data).unwrap().as_str())
    }

    pub fn set<T: Serialize>(&mut self, key: String, value: T) -> redis::RedisResult<String> {
        self.redis
            .set::<String, String, String>(key, serde_json::to_string(&value).unwrap())
    }

    pub fn get(&mut self, key: String) -> redis::RedisResult<String> {
        self.redis.get::<String, String>(key)
    }

    pub fn delete(&mut self, key: String) -> redis::RedisResult<String> {
        self.redis.del::<String, String>(key)
    }

    pub fn publish<T: Serialize>(&mut self, channel: String, data: T) -> RedisResult<i32> {
        self.redis
            .publish::<String, String, i32>(channel, serde_json::to_string(&data).unwrap())
            .into_redis_result()
    }
}
