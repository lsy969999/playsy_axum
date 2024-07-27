use axum::{middleware, routing::get, Router};
use tower::ServiceBuilder;

use crate::{configs::app_state::ArcAppState, controller::handlers::page::{board::{board_detail_page, board_edit_page, board_page}, game}, middlewares::auth::set_user_info_from_cookie_to_header};

pub fn get_board_router(state: ArcAppState) -> Router<ArcAppState>{
    Router::new()
        .route("/board", get(board_page))
        .route("/board/:board_sn", get(board_detail_page))
        .route("/board/edit", get(board_edit_page))
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn_with_state(state, set_user_info_from_cookie_to_header))
        )
}
