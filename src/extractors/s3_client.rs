use axum::{async_trait, extract::{FromRef, FromRequestParts}, http::request::Parts};
use hyper::StatusCode;

use crate::configs::app_state::ArcAppState;

pub struct AwsS3Client(pub aws_sdk_s3::Client);

#[async_trait]
impl<S> FromRequestParts<S> for AwsS3Client
where
    ArcAppState: FromRef<S>,
    aws_sdk_s3::Client: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let c = aws_sdk_s3::Client::from_ref(state);
        Ok(Self(c))
    }
}