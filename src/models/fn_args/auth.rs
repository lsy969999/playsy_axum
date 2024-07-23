use crate::models::oauth2::{GoogleOauth2UserInfo, NaverUserInfo};


pub struct EmailLoginArgs {
    pub email: String,
    pub password: String,
    pub addr: String,
    pub user_agent: String
}

pub struct GoogleLoginArgs<'a> {
    pub provider_access_token: Option<&'a str>,
    pub info: GoogleOauth2UserInfo,
    pub addr: String,
    pub user_agent: String,
}

pub struct NaverLoginArgs<'a> {
    pub provider_access_token: Option<&'a str>,
    pub provider_refresh_token: Option<&'a str>,
    pub info: NaverUserInfo,
    pub addr: String,
    pub user_agent: String,
}