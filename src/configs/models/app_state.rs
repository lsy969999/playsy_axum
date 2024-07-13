use bb8_redis::RedisConnectionManager;
use sqlx::{PgPool, Postgres};

/// 공유상태
#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: sqlx::Pool<Postgres>,
    pub redis_pool: bb8::Pool<RedisConnectionManager>,
}

impl AppState {
    pub fn new(
        db_pool: PgPool,
        redis_pool: bb8::Pool<RedisConnectionManager>
    ) -> Self {
        Self {
            db_pool,
            redis_pool,
        }
    }
}