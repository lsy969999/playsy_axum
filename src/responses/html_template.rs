use askama::Template;
use axum::response::{Html, IntoResponse, Response};
use crate::configs::errors::app_error::PageHandlerLayerError;

pub struct HtmlTemplate<T>(pub T);
impl <T> IntoResponse for HtmlTemplate<T> where T: Template{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => PageHandlerLayerError::Template(err).into_response()
        }
    }
}