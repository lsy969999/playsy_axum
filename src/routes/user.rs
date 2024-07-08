use std::sync::Arc;

use axum::{ routing::{get, post}, Router};

use crate::{handlers::user_page::{join_page, join_request, validate_nickname}, AppState};

pub fn get_user_router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/user", get_user_page_router())
}

fn get_user_page_router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/join",
            get(join_page)
                .post(join_request)
        )
}