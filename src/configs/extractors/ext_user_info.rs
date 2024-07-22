use std::convert::Infallible;
use axum::{async_trait, extract::{FromRef, FromRequestParts}, http::request::Parts};
use crate::{configs::{consts::USER_INFO, models::{app_state::ArcAppState, claims::AccessClaims}}, controller::handlers::page::fragment::user_info::UserInfo};

// pub struct UserInfo {
//     pub nick_name: String
// }

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
                                nick_name: claim.nick_name
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
