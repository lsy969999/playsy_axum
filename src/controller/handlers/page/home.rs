use askama::Template;
use axum::response::IntoResponse;

use crate::configs::into_responses::html_template::HtmlTemplate;


#[derive(Template)]
#[template(path="pages/home.html")]
struct HomeTemplate;

pub async fn home_page() -> impl IntoResponse{
    HtmlTemplate(HomeTemplate)
}
