use askama::Template;

#[derive(Template)]
#[template(path="pages/error.html")]
pub struct ErrorTemplate {
    pub error_code: String,
    pub error_message: String,
}