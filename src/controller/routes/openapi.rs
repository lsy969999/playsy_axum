use axum::{routing::get, Router};
use crate::{configs::models::app_state::ArcAppState, controller::handlers::api};

pub fn get_openapi_route() -> Router<ArcAppState> {
    Router::new()
        .route("/api-docs/openapi.json", get(api::openapi::openapi_handler))
}