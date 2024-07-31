use axum::{middleware::from_fn_with_state, routing::get, Router};
use tower::ServiceBuilder;

use crate::{configs::app_state::ArcAppState, controller::handlers::page::inquery::{inquery_page, inquery_upload}, middlewares::auth::set_user_info_from_cookie_to_header};

pub fn get_inquery_router(state: ArcAppState) -> Router<ArcAppState>{
    Router::new()
        .route("/inquery", get(inquery_page)
                                .post(inquery_upload))
        .layer(
            ServiceBuilder::new()
                .layer(from_fn_with_state(state, set_user_info_from_cookie_to_header))
        )
}
