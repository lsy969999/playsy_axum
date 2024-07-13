use std::sync::Arc;

use axum::{async_trait, extract::{FromRef, FromRequestParts}, http::request::Parts};
use bb8::{Pool, PooledConnection};
use bb8_redis::RedisConnectionManager;
use hyper::StatusCode;

use crate::configs::models::app_state::AppState;

pub struct RedisConnection(pub PooledConnection<'static, RedisConnectionManager>);

#[async_trait]
impl<S> FromRequestParts<S> for RedisConnection
where
    Arc<AppState>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = Arc::from_ref(state);
        let pool = Pool::from_ref(&state.redis_pool);
        let conn = pool.get_owned()
            .await
            .map_err(|err|(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

        Ok(Self(conn))
    }
}