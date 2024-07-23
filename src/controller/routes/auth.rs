use axum::{routing::{get, post}, Router};
use crate::{configs::app_state::ArcAppState, controller::handlers::page::auth::{apple_login, apple_login_callback, auth_page, discord_login, discord_login_callback, email_login, github_login, github_login_callback, google_login, google_login_callback, kakao_login, kakao_login_callback, logout, naver_login, naver_login_callback}};

pub fn get_auth_router() -> Router<ArcAppState> {
    Router::new()
        .nest("/auth", get_auth_page_router())
        .nest("/api/auth", get_auth_api_router())
}

fn get_auth_page_router() -> Router<ArcAppState> {
    Router::new()
        .route("/", get(auth_page))
        .route("/logout", get(logout))
        .route("/email/login", post(email_login))
        .route("/google/login", get(google_login))
        .route("/google/callback", get(google_login_callback))
        .route("/naver/login", get(naver_login))
        .route("/naver/callback", get(naver_login_callback))
        .route("/kakao/login", get(kakao_login))
        .route("/kakao/callback", get(kakao_login_callback))
        .route("/github/login", get(github_login))
        .route("/github/callback", get(github_login_callback))
        .route("/apple/login", get(apple_login))
        .route("/apple/callback", get(apple_login_callback))
        .route("/discord/login", get(discord_login))
        .route("/discord/callback", get(discord_login_callback))
}

fn get_auth_api_router() -> Router<ArcAppState> {
    Router::new()
        // .route("/request_token", post(api::auth::request_token))
}