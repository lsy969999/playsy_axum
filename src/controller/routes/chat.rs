use axum::{middleware, routing::get, Extension, Router};
use tower::ServiceBuilder;

use crate::{ configs::{app_extensions::WsChatExtension, app_state::ArcAppState}, controller::handlers::page::chat::{chat_page, ws_room_handler, ws_room_lobby_handler}, middlewares::auth::set_user_info_from_cookie_to_header};

pub fn get_chat_router(state: ArcAppState) -> Router<ArcAppState> {
    let ws_state: WsChatExtension = WsChatExtension::new();
    WsChatExtension::run_room_checker(ws_state.clone());
    Router::new()
        .route("/chat", get(chat_page))
        .route("/ws/chat/room/lobby", get(ws_room_lobby_handler))
        .route("/ws/chat/room/:roomid/:usersn", get(ws_room_handler))
        .layer(Extension(ws_state))
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn_with_state(state, set_user_info_from_cookie_to_header))
        )
}