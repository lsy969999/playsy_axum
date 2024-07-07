use std::sync::Arc;
use axum::{routing::{get, post}, Router};
use crate::{handlers::{auth_api::{protected_url, request_token}, auth_page::auth_page}, AppState};

pub fn get_auth_router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/", get_auth_page_router())
        .nest("/api/auth", get_auth_api_router())
}

fn get_auth_page_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/auth", get(auth_page))
}

fn get_auth_api_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/request_token", post(request_token))
        .route("/protected_url", get(protected_url))
}