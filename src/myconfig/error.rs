use axum::{extract::rejection::FormRejection, response::IntoResponse, Json};
use hyper::StatusCode;
use serde_json::json;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum AppError {
    #[error("test error: {0}")]
    TestError(String),
    #[error("CustomError500 error: {0}")]
    CustomError500(String),
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    #[error(transparent)]
    AxumFormRejection( #[from] FormRejection)
}

impl Into<(StatusCode, String)> for AppError {
    fn into(self) -> (StatusCode, String) {
        // error!("error!");
        match self {
            AppError::TestError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::CustomError500(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg.to_string()),
            AppError::AxumFormRejection(msg) => (StatusCode::BAD_REQUEST, msg.to_string()),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, body) = self.into();
        (status, body).into_response()
    }
}

// TODO!
#[derive(Error, Debug)]
pub enum AppPageError {
    #[error("Custom Page Error: {0}")]
    CustomPageError(String)
}






#[derive(Debug, Error)]
pub enum AuthError {
    #[error("WorongCredentials")]
    WorongCredentials,
    #[error("MissingCredentials")]
    MissingCredentials,
    #[error("TokenCreation")]
    TokenCreation,
    #[error("InvalidToken")]
    InvalidToken
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AuthError::WorongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credential"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}