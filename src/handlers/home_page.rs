use askama::Template;
use axum::response::IntoResponse;

use crate::into_responses::html_template::HtmlTemplate;

#[derive(Template)]
#[template(path="pages/home.html")]
struct HomeTemplate;

pub async fn home_page_handler() -> impl IntoResponse{
    HtmlTemplate(HomeTemplate)
}
