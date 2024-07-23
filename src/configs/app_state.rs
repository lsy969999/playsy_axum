use std::sync::Arc;
use axum::extract::FromRef;
use axum_csrf::CsrfConfig;
use bb8_redis::RedisConnectionManager;
use sqlx::PgPool;


/// 공유상태
#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub redis_pool: bb8::Pool<RedisConnectionManager>,
    pub csrf_config: CsrfConfig,
}

impl AppState {
    pub fn new(
        db_pool: PgPool,
        redis_pool: bb8::Pool<RedisConnectionManager>,
        csrf_config: CsrfConfig,
    ) -> Self {
        Self {
            db_pool,
            redis_pool,
            csrf_config,
        }
    }
}

// NewType Pattern
pub struct ArcAppState(pub Arc<AppState>);

impl ArcAppState {
    pub fn new(state: AppState) -> Self{
        Self(Arc::new(state))
    }
}

impl FromRef<ArcAppState> for Arc<AppState> {
    fn from_ref(input: &ArcAppState) -> Self {
        input.0.clone()
    }
}

impl FromRef<ArcAppState> for PgPool {
    fn from_ref(input: &ArcAppState) -> Self {
        input.0.db_pool.clone()
    }
}

impl FromRef<ArcAppState> for CsrfConfig {
    fn from_ref(input: &ArcAppState) -> Self {
        input.0.csrf_config.clone()
    }
}

impl FromRef<ArcAppState> for bb8::Pool<bb8_redis::RedisConnectionManager> {
    fn from_ref(input: &ArcAppState) -> Self {
        input.0.redis_pool.clone()
    }
}

impl Clone for ArcAppState {
    fn clone(&self) -> Self {
        ArcAppState(self.0.clone())
    }
}