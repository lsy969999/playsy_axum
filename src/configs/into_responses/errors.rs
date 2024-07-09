use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde_json::json;

use crate::configs::errors::auth::AuthError;

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            Self::MissingCredentials => (StatusCode::BAD_REQUEST, "MissingCredentials"),
            Self::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "TokenCreation"),
            Self::WrongCredential => (StatusCode::UNAUTHORIZED, "WrongCredential"),
            Self::UserNotExists => (StatusCode::INTERNAL_SERVER_ERROR, "UserNotExists"),
            Self::UserPasswordNotExists => (StatusCode::INTERNAL_SERVER_ERROR, "UserPasswordNotExists"),
            Self::Db => (StatusCode::INTERNAL_SERVER_ERROR, "dbError"),
            Self::PasswordVerify => (StatusCode::INTERNAL_SERVER_ERROR, "PasswordVerify"),
            Self::PasswordNotMatch => (StatusCode::INTERNAL_SERVER_ERROR, "PasswordNotMatch"),
        };
        let body = Json(json!({
            "error_message": error_message
        }));
        (status, body).into_response()
    }
}