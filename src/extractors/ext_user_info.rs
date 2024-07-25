use base64::prelude::*;
use std::convert::Infallible;
use axum::{async_trait, extract::{FromRef, FromRequestParts}, http::request::Parts};
use crate::{configs::{app_state::ArcAppState, consts::USER_INFO, errors::app_error::PageHandlerLayerError }, models::{claims::AccessClaims, user_info::UserInfo}};

/// 리퀘스트헤더로부터 사용자 정보를 읽는데
/// 사용자 정보가 없으면 401 에러 반환 한다.
/// 즉 반드시 사용자 정보가 있어야하는 핸들러에서 사용하도록 한다.
pub struct UserInfoForPage(pub UserInfo);

/// 리퀘스트헤더로부터 사용자 정보를 읽는데
/// 사용자 정보가 없으면 None, 있으면 Some(UserInfo)를 넣어준다.
/// 사용자 정보가 있으니 없으나 상관없는곳에서 사용하도록 한다.
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
                        // headervalue -> str
                        let str = uf.to_str()?;
                        // str -> user_info_str
                        let user_info_str = String::from_utf8(BASE64_STANDARD.decode(str)?)?;
                        // user_info_str -> acc claim
                        let claim = serde_json::from_str::<AccessClaims>(&user_info_str)?;
                        // acc claim -> user_info struct
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
            Err(err) => {
                tracing::error!("ExtUserInfo err! err: {:?}", err);
                None
            },
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
