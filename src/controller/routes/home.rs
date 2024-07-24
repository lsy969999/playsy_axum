use axum::{middleware::from_fn_with_state, routing::get, Router};
use tower::ServiceBuilder;
use crate::{ configs::app_state::ArcAppState, controller::handlers::page, middlewares::auth::set_user_info_from_cookie_to_header};

pub fn get_home_router(state: ArcAppState) -> Router<ArcAppState>{
    Router::new()
        .route("/", get(page::home::home_page))
        .route("/privacy", get(page::home::privacy_page))
        .layer(
            ServiceBuilder::new()
                .layer(from_fn_with_state(state, set_user_info_from_cookie_to_header))
        )
}

#[allow(dead_code)]
fn get_page_router() {
    todo!()
}

#[allow(dead_code)]
fn get_api_router() {
    todo!()
}