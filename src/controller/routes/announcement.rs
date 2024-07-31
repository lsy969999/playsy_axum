use axum::{middleware::from_fn_with_state, routing::get, Router};
use tower::ServiceBuilder;

use crate::{configs::app_state::ArcAppState, controller::handlers::page::announcement::{announcement_detail_page, announcement_page}, middlewares::auth::set_user_info_from_cookie_to_header};

pub fn get_announcement_router(state: ArcAppState) -> Router<ArcAppState>{
    Router::new()
        .route("/announcement", get(announcement_page))
        .route("/announcement/:announcement_id", get(announcement_detail_page))
        .layer(
            ServiceBuilder::new()
                .layer(from_fn_with_state(state, set_user_info_from_cookie_to_header))
        )
}
