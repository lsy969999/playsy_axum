use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{configs::models::app_state::AppState, controller::handlers::page::game};

pub fn get_game_router() -> Router<Arc<AppState>>{
    Router::new()
        .route("/game/bevy_wasm_test", get(game::bevy_wasm_test_page))
        
}