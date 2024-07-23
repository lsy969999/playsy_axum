use axum::{routing::{get, post}, Router};
use crate::{configs::app_state::ArcAppState, controller::handlers::page};

pub fn get_auth_router() -> Router<ArcAppState> {
    Router::new()
        .nest("/auth", get_auth_page_router())
        .nest("/api/auth", get_auth_api_router())
}

fn get_auth_page_router() -> Router<ArcAppState> {
    Router::new()
        .route("/", get(page::auth::auth_page))
        .route("/email", post(page::auth::auth_email_request))
        .route("/logout", post(page::auth::logout))
        .route("/google/login", get(page::auth::google_login))
        .route("/google/callback", get(page::auth::google_callback))
        .route("/naver/login", get(page::auth::naver_login))
        .route("/naver/callback", get(page::auth::naver_callback))
        .route("/github/login", get(page::auth::github_login))
        .route("/github/callback", get(page::auth::github_callback))
}

fn get_auth_api_router() -> Router<ArcAppState> {
    Router::new()
        // .route("/request_token", post(api::auth::request_token))
}