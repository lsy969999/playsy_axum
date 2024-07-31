use axum::response::IntoResponse;

use crate::{extractors::ext_user_info::ExtUserInfo, responses::html_template::HtmlTemplate, templates::about::AboutTemplate};

pub async fn about_page(
    ExtUserInfo(user_info): ExtUserInfo
) -> impl IntoResponse {
    HtmlTemplate(
        AboutTemplate {
            user_info
        }
    )
}