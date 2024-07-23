use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde_json::json;
use crate::{configs::errors::app_error::{ApiHandlerLayerError, PageHandlerLayerError, RepositoryLayerError, ServiceLayerError},  responses::html_template::HtmlTemplate, templates::error::ErrorTemplate};

impl IntoResponse for PageHandlerLayerError {
    fn into_response(self) -> axum::response::Response {
        const TAG: &str = "[PageHandlerLayerError]";
        let (statue, error_message) = match &self {
            Self::Service(ServiceLayerError::Repository(RepositoryLayerError::Db(err)))=> {
                tracing::error!("{TAG} repository {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR")
            }
            Self::Service(ServiceLayerError::Jwt(err)) => {
                tracing::error!("{TAG} jwt {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR")
            }
            Self::Service(ServiceLayerError::CustomUser(err)) => {
                tracing::error!("{TAG} custom_user {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR")
            }
            Self::Service(ServiceLayerError::CustomCrypto(err)) => {
                tracing::error!("{TAG} custom_crypto {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR")
            }
            Self::Service(ServiceLayerError::CustomAuth(err)) => {
                tracing::error!("{TAG} custom_auth {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR")
            }
            Self::Service(ServiceLayerError::ParseJson(err)) => {
                tracing::error!("{TAG} parse_json {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR")
            }
            Self::Csrf(err) => {
                tracing::error!("{TAG} csrf {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR")
            }
            Self::Auth => {
                tracing::error!("{TAG} auth");
                (StatusCode::UNAUTHORIZED, "UNAUTHORIZED")
            }
            Self::Any(err) => {
                tracing::error!("{TAG} any {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR")
            }
        };
        HtmlTemplate(
            ErrorTemplate {
                error_code: statue.to_string(),
                error_message: error_message.to_string(),
            }
        ).into_response()
    }
}

impl IntoResponse for ApiHandlerLayerError {
    fn into_response(self) -> axum::response::Response {
        let error_message = "";
        let _body = Json(json!({
            "error_message": error_message
        }));
        todo!()
    }
}

