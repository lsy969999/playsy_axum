use askama::Template;
use axum::response::IntoResponse;
use crate::configs::{extractors::ext_user_info::ExtUserInfo, into_responses::html_template::HtmlTemplate};
use super::fragment::user_info::UserInfo;

#[derive(Template)]
#[template(path="pages/home.html")]
struct HomeTemplate {
    user_info: Option<UserInfo>
}

pub async fn home_page(
    ExtUserInfo(user_info): ExtUserInfo,
) -> impl IntoResponse{
    HtmlTemplate(
        HomeTemplate{
            user_info
        }
    )
}

#[derive(Template)]
#[template(path="pages/privacy.html")]
struct PrivacyTemplate {
    user_info: Option<UserInfo>
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