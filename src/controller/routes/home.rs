use axum::{middleware::from_fn_with_state, routing::get, Router};
use tower::ServiceBuilder;
use crate::{configs::{middlewares::auth::set_user_info_from_cookie_to_header, models::app_state::ArcAppState}, controller::handlers::page};

pub fn get_home_router(state: ArcAppState) -> Router<ArcAppState>{
    Router::new()
        .route("/", get(page::home::home_page))
        .layer(
            ServiceBuilder::new()
                .layer(from_fn_with_state(state, set_user_info_from_cookie_to_header))
        )
}