use axum::{middleware::from_fn_with_state, routing::get, Router};
use tower::ServiceBuilder;

use crate::{configs::app_state::ArcAppState, controller::handlers::page::about::about_page, middlewares::auth::set_user_info_from_cookie_to_header};

pub fn get_about_router(state: ArcAppState) -> Router<ArcAppState>{
    Router::new()
        .route("/about", get(about_page))
        .layer(
            ServiceBuilder::new()
                .layer(from_fn_with_state(state, set_user_info_from_cookie_to_header))
        )
}
