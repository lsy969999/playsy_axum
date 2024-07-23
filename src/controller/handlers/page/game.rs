
use axum::response::IntoResponse;
use crate::{extractors::ext_user_info::ExtUserInfo, responses::html_template::HtmlTemplate, templates::game::BevyWasmTestTemplate};


pub async fn bevy_wasm_test_page(
    ExtUserInfo(user_info): ExtUserInfo,
) -> impl IntoResponse {
    HtmlTemplate(
        BevyWasmTestTemplate {
            user_info: user_info
        }
    )
}