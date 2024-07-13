use std::time::Duration;
use bb8::PooledConnection;
use bb8_redis::RedisConnectionManager;
use redis::{AsyncCommands, FromRedisValue, RedisError};
use tokio::time::timeout;

pub async fn get<T>(
    conn: &mut PooledConnection<'_, RedisConnectionManager>,
    key: &str
) -> Result<T, redis::RedisError>
    where T: FromRedisValue {
    let timeout_duration = Duration::from_secs(5); //TODO! sec
    let v : Result<T, RedisError> = timeout(timeout_duration, conn.get(key))
        .await
        .map_err(|_e| RedisError::from((redis::ErrorKind::IoError, "Timeout occurred")))?;
    Ok(v?)
}

pub async fn set(
    conn: &mut PooledConnection<'_, RedisConnectionManager>,
    key: &str,
    value: &str,
) -> Result<(), redis::RedisError> {
    let timeout_duration = Duration::from_secs(5); //TODO! sec
    timeout(timeout_duration, conn.set(key, value))
        .await
        .map_err(|_e| RedisError::from((redis::ErrorKind::IoError, "Timeout occurred")))??;
    Ok(())
}