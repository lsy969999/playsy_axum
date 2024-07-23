use axum::{middleware, routing::get, Router};
use tower::ServiceBuilder;

use crate::{ configs::app_state::ArcAppState, controller::handlers::page::game, middlewares::auth::set_user_info_from_cookie_to_header};

pub fn get_game_router(state: ArcAppState) -> Router<ArcAppState>{
    Router::new()
        .route("/game/bevy_wasm_test", get(game::bevy_wasm_test_page))
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn_with_state(state, set_user_info_from_cookie_to_header))
        )
}