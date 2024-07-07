use askama::Template;
use axum::response::IntoResponse;

use crate::myconfig::into_response::HtmlTemplate;

#[derive(Template)]
#[template(path="pages/home.html")]
struct HomeTemplate;

pub async fn home_page_handler() -> impl IntoResponse{
    HtmlTemplate(HomeTemplate)
}
