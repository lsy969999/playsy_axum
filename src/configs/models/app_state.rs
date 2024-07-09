use sqlx::PgPool;

/// 공유상태
#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}

impl AppState {
    pub fn new(db_pool: PgPool) -> Self {
        Self {
            db_pool,
        }
    }
}