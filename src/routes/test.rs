use std::sync::Arc;

use axum::{routing::{get, post}, Router};
use crate::{handlers::test::{test_validate_handler, using_connection_pool_extractor2}, AppState};

pub fn get_test_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/validate_test", post(test_validate_handler))
        .route("/db_test", get(using_connection_pool_extractor2))
}