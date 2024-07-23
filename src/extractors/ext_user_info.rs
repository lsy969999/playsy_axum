use std::convert::Infallible;
use axum::{async_trait, extract::{FromRef, FromRequestParts}, http::request::Parts};
use crate::{configs::{app_state::ArcAppState, consts::USER_INFO, errors::app_error::PageHandlerLayerError }, models::{claims::AccessClaims, user_info::UserInfo}};

// pub struct UserInfo {
//     pub nick_name: String
// }

pub struct UserInfoForPage(pub UserInfo);

pub struct ExtUserInfo(pub Option<UserInfo>);

#[async_trait]
impl<S> FromRequestParts<S> for ExtUserInfo
where
    ArcAppState: FromRef<S>,
    S: Send + Sync
{
    type Rejection = Infallible;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let uf: anyhow::Result<Option<UserInfo>> = (|| {
            Ok(
                match parts.headers.get(USER_INFO) {
                    Some(uf) => {
                        let str = uf.to_str()?;
                        let claim = serde_json::from_str::<AccessClaims>(str)?;
                        Some(
                            UserInfo {
                                user_sn: claim.sub.parse().unwrap(),
                                nick_name: claim.nick_name,
                                avatar_url: claim.avatar_url,
                            }
                        )
                    }
                    None => None
                }
            )
        })();

        let uf = match uf {
            Ok(uf) => uf,
            Err(_) => None,
        };

        Ok(Self(uf))
    }
}


#[async_trait]
impl<S> FromRequestParts<S> for UserInfoForPage
where
    ArcAppState: FromRef<S>,
    S: Send + Sync
{
    type Rejection = PageHandlerLayerError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let ExtUserInfo(uf) = ExtUserInfo::from_request_parts(parts, _state).await.unwrap();
        Ok(
            Self(
                match uf {
                    Some(uf) => uf,
                    None => {
                        Err(PageHandlerLayerError::Auth)?
                    }
                }
            )
        )
    }
}
