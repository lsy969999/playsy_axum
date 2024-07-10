use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde_json::json;

use crate::configs::errors::app_error::{ApiHandlerLayerError, PageHandlerLayerError, ServiceLayerError};

// impl IntoResponse for AuthError {
//     fn into_response(self) -> axum::response::Response {
//         let (status, error_message) = match self {
//             Self::MissingCredentials => (StatusCode::BAD_REQUEST, "MissingCredentials"),
//             Self::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "TokenCreation"),
//             Self::WrongCredential => (StatusCode::UNAUTHORIZED, "WrongCredential"),
//             Self::UserNotExists => (StatusCode::INTERNAL_SERVER_ERROR, "UserNotExists"),
//             Self::UserPasswordNotExists => (StatusCode::INTERNAL_SERVER_ERROR, "UserPasswordNotExists"),
//             Self::Db => (StatusCode::INTERNAL_SERVER_ERROR, "dbError"),
//             Self::PasswordVerify => (StatusCode::INTERNAL_SERVER_ERROR, "PasswordVerify"),
//             Self::PasswordNotMatch => (StatusCode::INTERNAL_SERVER_ERROR, "PasswordNotMatch"),
//         };
//         let body = Json(json!({
//             "error_message": error_message
//         }));
//         (status, body).into_response()
//     }
// }

impl IntoResponse for PageHandlerLayerError {
    fn into_response(self) -> axum::response::Response {
        let (statue, error_message) = match self {
            Self::Service(ServiceLayerError::Repository(_))=> {
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR")
            }
            Self::Service(ServiceLayerError::Jwt(_)) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR")
            }
            Self::Service(ServiceLayerError::CustomUser(_)) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR")
            }
            Self::Service(ServiceLayerError::CustomCrypto(_)) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR")
            }
            Self::Service(ServiceLayerError::CustomAuth(_)) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR")
            }
        };
        let body = Json(json!({
            "error_message": error_message
        }));
        (statue, body).into_response()
    }
}

impl IntoResponse for ApiHandlerLayerError {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}