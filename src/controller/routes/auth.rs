use std::sync::Arc;
use axum::{routing::get, Router};

use crate::{configs::models::app_state::AppState, controller::handlers::page};

pub fn get_auth_router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/auth", get_auth_page_router())
        .nest("/api/auth", get_auth_api_router())
}

fn get_auth_page_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/",
            get(page::auth::auth_page)
            .post(page::auth::auth_request)
        )
}

fn get_auth_api_router() -> Router<Arc<AppState>> {
    Router::new()
        // .route("/request_token", post(api::auth::request_token))
}