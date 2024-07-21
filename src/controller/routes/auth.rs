use std::sync::Arc;
use axum::{routing::{get, post}, Router};

use crate::{configs::models::app_state::AppState, controller::handlers::page};

pub fn get_auth_router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/auth", get_auth_page_router())
        .nest("/api/auth", get_auth_api_router())
}

fn get_auth_page_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(page::auth::auth_page))
        .route("/email", post(page::auth::auth_email_request))
        .route("/logout", post(page::auth::logout))
        .route("/google/login", get(page::auth::google_login))
        .route("/google/callback", get(page::auth::google_callback))
}

fn get_auth_api_router() -> Router<Arc<AppState>> {
    Router::new()
        // .route("/request_token", post(api::auth::request_token))
}