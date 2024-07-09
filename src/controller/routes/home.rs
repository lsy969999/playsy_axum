
use std::sync::Arc;
use axum::{routing::get, Router};

use crate::{configs::models::app_state::AppState, controller::handlers::page};

pub fn get_home_router() -> Router<Arc<AppState>>{
    Router::new()
        .route("/", get(page::home::home_page))
}