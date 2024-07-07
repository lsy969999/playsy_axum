
use std::sync::Arc;
use axum::{routing::get, Router};
use crate::{handlers::home_page::home_page_handler, AppState};

pub fn get_home_router() -> Router<Arc<AppState>>{
    Router::new()
        .route("/", get(home_page_handler))
}