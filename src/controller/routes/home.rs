
use std::sync::Arc;
use axum::{middleware::{from_fn, from_fn_with_state}, routing::get, Router};
use tower::ServiceBuilder;

use crate::{configs::{middlewares::auth::{set_user_info_from_cookie_to_header, validate_user_info_from_header}, models::app_state::AppState}, controller::handlers::page};

pub fn get_home_router(state: Arc<AppState>) -> Router<Arc<AppState>>{

    Router::new()
        .route("/", get(page::home::home_page))
        .layer(
            ServiceBuilder::new()
                .layer(from_fn_with_state(state, set_user_info_from_cookie_to_header))
                // .layer(from_fn(validate_user_info_from_header))
        )
}