use axum::response::IntoResponse;
use crate::{extractors::ext_user_info::ExtUserInfo, responses::html_template::HtmlTemplate, templates::home::{HomeTemplate, PrivacyTemplate}};
pub async fn home_page(
    ExtUserInfo(user_info): ExtUserInfo,
) -> impl IntoResponse{
    HtmlTemplate(
        HomeTemplate{
            user_info
        }
    )
}

pub async fn privacy_page(
    ExtUserInfo(user_info): ExtUserInfo,
) -> impl IntoResponse {
    HtmlTemplate(
        PrivacyTemplate {
            user_info
        }
    )
}