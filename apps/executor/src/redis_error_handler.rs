use log::error;
use redis::RedisError;

pub(crate) fn handle_redis_error(err: RedisError, thread_name: String, task_name: &str) {
    if err.is_io_error() {
        error!("[{}][{}] redis error: {:?}", thread_name, task_name, err);
    }
}
