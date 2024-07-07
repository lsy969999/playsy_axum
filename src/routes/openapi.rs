use std::sync::Arc;

use axum::{routing::get, Router};
use crate::{handlers::openapi::openapi_handler, AppState};

pub fn get_openapi_route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api-docs/openapi.json", get(openapi_handler))
}