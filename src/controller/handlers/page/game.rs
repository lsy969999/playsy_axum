use askama::Template;
use axum::response::IntoResponse;

use crate::configs::into_responses::html_template::HtmlTemplate;

use super::fragment::user_info::UserInfo;

#[derive(Template)]
#[template(path="pages/games/bevy_wasm_test.html")]
struct BevyWasmTestTemplate{
    user_info: Option<UserInfo>
}

pub async fn bevy_wasm_test_page() -> impl IntoResponse {
    HtmlTemplate(
        BevyWasmTestTemplate {
            user_info: None
        }
    )
}