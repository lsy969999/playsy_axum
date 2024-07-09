use std::sync::Arc;
use axum::{routing::get, Router};

use crate::{configs::models::app_state::AppState, controller::handlers::api};

pub fn get_openapi_route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api-docs/openapi.json", get(api::openapi::openapi_handler))
}