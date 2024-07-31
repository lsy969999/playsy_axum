use axum::{extract::{Path, Query}, response::IntoResponse};
use crate::{configs::errors::app_error::PageHandlerLayerError, extractors::{database_connection::DatabaseConnection, ext_user_info::ExtUserInfo}, models::request::pagination::PaginationReq, responses::html_template::HtmlTemplate, services, templates::announcement::{AnnouncementDetailTemplate, AnnouncementTemplate}};

pub async fn announcement_page(
    DatabaseConnection(mut conn): DatabaseConnection,
    ExtUserInfo(user_info): ExtUserInfo,
    Query(query): Query<PaginationReq>,
) -> Result<impl IntoResponse, PageHandlerLayerError> {
    let (announcements, pagination) = services::announcement::get_announcements(&mut conn, &query).await?;
    Ok(
        HtmlTemplate(
            AnnouncementTemplate{
                user_info,
                announcements,
                pagination
            }
        )
    )
}

pub async fn announcement_detail_page(
    DatabaseConnection(mut conn): DatabaseConnection,
    Path(announcement_sn): Path<i32>,
    ExtUserInfo(user_info): ExtUserInfo
) -> Result<impl IntoResponse, PageHandlerLayerError> {
    let ann = services::announcement::get_announcement(&mut conn, announcement_sn).await?;
    Ok(
        HtmlTemplate(
            AnnouncementDetailTemplate {
                user_info,
                announcement: ann,
            }
        )
    )
}