use axum::{ middleware::from_fn_with_state, routing::{get, post}, Router};
use tower::ServiceBuilder;
use crate::{configs::app_state::ArcAppState, controller::handlers::page::{self, user::join_email_success_page}, middlewares::auth::set_user_info_from_cookie_to_header};


pub fn get_user_router(state: ArcAppState) -> Router<ArcAppState> {
    Router::new()
        .nest("/user", get_user_page_router(state))
}

fn get_user_page_router(state:ArcAppState ) -> Router<ArcAppState> {
    Router::new()
        .route("/join_email", get(page::user::join_email_page)
                            .post(page::user::email_join_request))
        .route("/join_email_success", get(join_email_success_page))
        .route("/join_social", get(page::user::join_social_page))
        .route("/withdrawl", post(page::user::user_withdrawl))
        .route("/mypage", get(page::user::my_page))
        .route("/nick_validate", get(page::user::nick_validate))
        .route("/email_validate", get(page::user::email_validate))
        .route("/nick_update", post(page::user::user_nick_name_update))
        .layer(
            ServiceBuilder::new()
                .layer(from_fn_with_state(state, set_user_info_from_cookie_to_header))
        )
}