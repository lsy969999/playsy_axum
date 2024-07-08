use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde_json::json;

use crate::errors::auth::MyAuthError;

impl IntoResponse for MyAuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            Self::MissingCredentials => (StatusCode::BAD_REQUEST, "MissingCredentials"),
            Self::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "TokenCreation"),
            Self::WrongCredential => (StatusCode::UNAUTHORIZED, "WrongCredential"),
        };
        let body = Json(json!({
            "error_message": error_message
        }));
        (status, body).into_response()
    }
}