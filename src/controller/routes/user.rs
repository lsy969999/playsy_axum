use axum::{ middleware::from_fn_with_state, routing::{get, post}, Router};
use tower::ServiceBuilder;
use crate::{configs::{middlewares::auth::set_user_info_from_cookie_to_header, models::app_state::ArcAppState}, controller::handlers::page};


pub fn get_user_router(state: ArcAppState) -> Router<ArcAppState> {
    Router::new()
        .nest("/user", get_user_page_router(state))
}

fn get_user_page_router(state:ArcAppState ) -> Router<ArcAppState> {
    Router::new()
        .route(
            "/join",
            get(page::user::join_page)
                .post(page::user::join_request),
        )
        .route("/withdrawl", post(page::user::user_withdrawl))
        .route("/mypage", get(page::user::my_page))
        .route("/nick_validate", get(page::user::nick_validate))
        .route("/email_validate", get(page::user::email_validate))
        .layer(
            ServiceBuilder::new()
                .layer(from_fn_with_state(state, set_user_info_from_cookie_to_header))
        )
}