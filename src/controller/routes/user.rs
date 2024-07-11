use std::sync::Arc;
use axum::{ routing::get, Router};

use crate::{configs::models::app_state::AppState, controller::handlers::page};


pub fn get_user_router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/user", get_user_page_router())
}

fn get_user_page_router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/join",
            get(page::user::join_page)
                .post(page::user::join_request),
        )
        .route("/nick_validate", get(page::user::nick_validate))
        .route("/email_validate", get(page::user::email_validate))
}