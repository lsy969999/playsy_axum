use std::{convert::Infallible, time::Duration};
use axum::{async_trait, extract::{FromRef, FromRequestParts}, http::request::Parts};
use bb8::{Pool, PooledConnection};
use bb8_redis::RedisConnectionManager;
use redis::RedisError;
use tokio::time::timeout;

use crate::configs::app_state::ArcAppState;

pub struct RedisConnection(pub Option<PooledConnection<'static, RedisConnectionManager>>);

#[async_trait]
impl<S> FromRequestParts<S> for RedisConnection
where
    ArcAppState: FromRef<S>,
    Pool<RedisConnectionManager>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // let state = Arc::from_ref(state);
        let pool = Pool::from_ref(state);
        let conn = pool.get_owned();
        let timeout_duration = Duration::from_secs(5); //TODO! sec정하기
        let timeout_conn = timeout(timeout_duration, conn)
            .await
            .map_err(|_e| RedisError::from((redis::ErrorKind::IoError, "Timeout occurred")));
        let res = match timeout_conn {
            Ok(Ok(conn)) => Some(conn),
            Ok(Err(err)) => {
                tracing::error!("Redis Get From Pool errir check need! err: {}", err);
                None
            }
            Err(err) => {
                tracing::error!("Redis Get From Pool errir check need! err: {}", err);
                None
            }
        };
        Ok(Self(res))
    }
}