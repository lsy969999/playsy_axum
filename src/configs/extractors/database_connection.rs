use std::sync::Arc;

use axum::{async_trait, extract::{FromRef, FromRequestParts}, http::request::Parts};
use hyper::StatusCode;
use sqlx::PgPool;

use crate::configs::models::app_state::AppState;



pub struct DatabaseConnection(pub sqlx::pool::PoolConnection<sqlx::Postgres>);
#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    Arc<AppState>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = Arc::from_ref(state);
        let pool = PgPool::from_ref(&state.db_pool);
        let conn = pool.acquire()
            .await
            .map_err(|err|(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

        Ok(Self(conn))
    }
}