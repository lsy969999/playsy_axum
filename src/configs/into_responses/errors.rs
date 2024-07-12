use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde_json::json;
use crate::configs::errors::app_error::{ApiHandlerLayerError, PageHandlerLayerError, ServiceLayerError};

impl IntoResponse for PageHandlerLayerError {
    fn into_response(self) -> axum::response::Response {
        const TAG: &str = "[PageHandlerLayerError] ServiceLayerError::Repository";
        let (statue, error_message) = match self {
            Self::Service(ServiceLayerError::Repository(err))=> {
                tracing::error!("{TAG} {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR")
            }
            Self::Service(ServiceLayerError::Jwt(err)) => {
                tracing::error!("{TAG} {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR")
            }
            Self::Service(ServiceLayerError::CustomUser(err)) => {
                tracing::error!("{TAG} {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR")
            }
            Self::Service(ServiceLayerError::CustomCrypto(err)) => {
                tracing::error!("{TAG} {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR")
            }
            Self::Service(ServiceLayerError::CustomAuth(err)) => {
                tracing::error!("{TAG} {}", err);
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