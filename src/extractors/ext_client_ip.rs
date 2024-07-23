use std::{convert::Infallible, net::SocketAddr};
use axum::{async_trait, extract::{ConnectInfo, FromRef, FromRequestParts}, http::request::Parts};

use crate::configs::app_state::ArcAppState;


pub struct ExtClientIp(pub Option<String>);

// TODO: 프록시 넘어오는 X-Forward-For 에대해서 처리
#[async_trait]
impl<S> FromRequestParts<S> for ExtClientIp
where
   ArcAppState: FromRef<S>,
    S: Send + Sync
{
    type Rejection = Infallible;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let connect_info = ConnectInfo::<SocketAddr>::from_request_parts(parts, state).await;
        let res: Option<String> = match connect_info {
            Ok(ConnectInfo(addr)) => {
                Some(addr.to_string())
            }
            Err(err) => {
                tracing::error!("ExtClientIp error! {:?}, parts: {:?}", err, parts);
                None
            }
        };
        Ok(Self(res))
    }
}