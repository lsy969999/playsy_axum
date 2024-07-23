use askama::Template;
use axum::response::IntoResponse;
use crate::configs::{extractors::ext_user_info::ExtUserInfo, into_responses::html_template::HtmlTemplate, models::user_info::UserInfo};

#[derive(Template)]
#[template(path="pages/games/bevy_wasm_test.html")]
struct BevyWasmTestTemplate{
    user_info: Option<UserInfo>
}

pub async fn bevy_wasm_test_page(
    ExtUserInfo(user_info): ExtUserInfo,
) -> impl IntoResponse {
    HtmlTemplate(
        BevyWasmTestTemplate {
            user_info: user_info
        }
    )
}