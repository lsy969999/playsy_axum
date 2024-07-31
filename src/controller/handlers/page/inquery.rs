use axum::{response::IntoResponse, Form};

use crate::{configs::errors::app_error::PageHandlerLayerError, extractors::{database_connection::DatabaseConnection, ext_user_info::ExtUserInfo}, models::request::inquery::InqueryReqDto, responses::html_template::HtmlTemplate, services, templates::inquery::{InquerySuccessFragment, InqueryTemplate}};

pub async fn inquery_page(
    ExtUserInfo(user_info): ExtUserInfo
) -> impl IntoResponse {
    HtmlTemplate(
        InqueryTemplate {
            user_info
        }
    )
}

pub async fn inquery_upload(
    DatabaseConnection(mut conn): DatabaseConnection,
    ExtUserInfo(user_info): ExtUserInfo,
    Form(form): Form<InqueryReqDto>,
) -> Result<impl IntoResponse, PageHandlerLayerError> {
    tracing::debug!("asdf");
    let user_sn = match user_info {
        Some(user_info) => Some(user_info.user_sn),
        None => None
    };
    services::inquery::inquery_insert(&mut conn, user_sn, form.email.as_deref(), form.subject.as_deref(),form.message.as_deref()).await?;
    Ok((HtmlTemplate(InquerySuccessFragment)).into_response())
}