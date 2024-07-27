use axum::response::IntoResponse;
use crate::{extractors::ext_user_info::{ExtUserInfo, UserInfoForPage}, responses::html_template::HtmlTemplate, templates::board::{BoardDetailTemplate, BoardEditTemplate, BoardTemplate}};

pub async fn board_page(
    ExtUserInfo(user_info): ExtUserInfo,
) -> impl IntoResponse {
    HtmlTemplate(
        BoardTemplate {
            user_info
        }
    )
}

pub async fn board_detail_page(
    ExtUserInfo(user_info): ExtUserInfo,
) -> impl IntoResponse {
    HtmlTemplate(
        BoardDetailTemplate {
            user_info
        }
    )
}

pub async fn board_edit_page(
    UserInfoForPage(user_info): UserInfoForPage,
) -> impl IntoResponse {
    HtmlTemplate(
        BoardEditTemplate {
            user_info: Some(user_info)
        }
    )
}