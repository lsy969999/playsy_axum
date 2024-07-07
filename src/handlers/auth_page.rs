
use askama::Template;
use axum::response::IntoResponse;
use crate::myconfig::into_response::HtmlTemplate;

#[derive(Template)]
#[template(path="auth_page.html")]
struct AuthTemplate;

pub async fn auth_page() -> impl IntoResponse {
    HtmlTemplate(
        AuthTemplate
    )
}
