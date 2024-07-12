use std::{convert::Infallible, sync::Arc};
use axum::{async_trait, extract::{FromRef, FromRequestParts}, http::request::Parts};
use axum_extra::extract::CookieJar;
use crate::{configs::{consts::ACCESS_TOKEN, models::app_state::AppState}, controller::handlers::page::fragment::user_info::UserInfo, utils};

// pub struct UserInfo {
//     pub nick_name: String
// }

pub struct ExtUserInfo(pub Option<UserInfo>);

#[async_trait]
impl<S> FromRequestParts<S> for ExtUserInfo
where
    Arc<AppState>: FromRef<S>,
    S: Send + Sync
{
    type Rejection = Infallible;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, state).await.unwrap();
        let at_cookie = jar.get(ACCESS_TOKEN);
        match at_cookie {
           Some(cookie) => {
                let token_str = cookie.value();
                let acc_keys = utils::settings::get_settings_jwt_access_keys();
                let claim = utils::jwt::decode_jwt(token_str, &acc_keys.decoding);
                match claim {
                    Ok(claim) => {
                        let uinfo = UserInfo{ nick_name: claim.sub.to_string() };
                        Ok(Self(Some(uinfo)))
                    }
                    Err(error) => {
                        tracing::error!("ExtUserInfo decode error: {:?}", error);
                        Ok(Self(None))
                    }
                }
           }
           None => Ok(Self(None))
        }
    }
}
