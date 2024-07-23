use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, Scope, TokenUrl};
use reqwest::Client;
use serde::Deserialize;
use validator::Validate;

use crate::{configs::etc::oauth2_naver_client::NaverClient, validators::nick_name_vali_char};


pub fn google_oauth2_client() -> BasicClient {
    let o = super::config::get_config_oauth2();
    BasicClient::new(
        ClientId::new(format!("{}",o.oauth2_google_client_id)),
        Some(ClientSecret::new(format!("{}", o.oauth2_google_client_secret))),
        AuthUrl::new(format!("https://accounts.google.com/o/oauth2/v2/auth")).unwrap(),
        Some(TokenUrl::new(format!("https://oauth2.googleapis.com/token")).unwrap())
    )
    .set_redirect_uri(RedirectUrl::new(format!("http://localhost:4000/auth/google/callback")).unwrap())
}

pub fn google_oauth2_scope_profile() -> Scope {
    Scope::new(format!("https://www.googleapis.com/auth/userinfo.profile"))
}

pub fn google_oauth2_scope_email() -> Scope {
    Scope::new(format!("https://www.googleapis.com/auth/userinfo.email"))
}

#[derive(Debug, Deserialize, Validate)]
pub struct GoogleOauth2UserInfo {
    pub sub: Option<String>,
    #[validate(length(min = 3, max=10, message = "닉네임은 3글자 이상 10글자 미만 이어야 합니다."))]
    #[validate(custom(function="nick_name_vali_char"))]
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub picture: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>
}

pub async fn google_oauth2_user_info_api(access_token: &str) -> anyhow::Result<GoogleOauth2UserInfo> {
    let url = "https://www.googleapis.com/oauth2/v3/userinfo";

    let client = Client::new();
    let response = client
        .get(url)
        .bearer_auth(access_token)
        .send()
        .await?;

    Ok(response.json::<GoogleOauth2UserInfo>().await?)
}

//

pub async fn naver_oauth2_client() -> NaverClient {
    let o = super::config::get_config_oauth2();
    NaverClient::new(
        ClientId::new(format!("{}",o.oauth2_naver_client_id)),
        Some(ClientSecret::new(format!("{}", o.oauth2_naver_client_secret))),
        AuthUrl::new(format!("https://nid.naver.com/oauth2.0/authorize")).unwrap(),
        Some(TokenUrl::new(format!("https://nid.naver.com/oauth2.0/token")).unwrap())
    )
    .set_redirect_uri(RedirectUrl::new(format!("http://localhost:4000/auth/naver/callback")).unwrap())
}

#[derive(Debug, Deserialize, Validate)]
pub struct NaverUserInfo {
    pub id: String,
    pub name: Option<String>,
    pub email: String,
    #[validate(length(min = 3, max=10, message = "닉네임은 3글자 이상 10글자 미만 이어야 합니다."))]
    #[validate(custom(function="nick_name_vali_char"))]
    pub nickname: Option<String>,
    pub profile_image: Option<String>,
    // age: String,
    // gender: String,
    // birthday: String,
    // birthyear: String,
    // mobile: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NaverResponse {
    resultcode: String,
    message: String,
    response: NaverUserInfo,
}

pub async fn naver_oauth2_user_info_api(access_token: &str) -> anyhow::Result<NaverUserInfo> {
    let url = "https://openapi.naver.com/v1/nid/me";

    let client = Client::new();
    let response = client
        .get(url)
        .bearer_auth(access_token)
        .send()
        .await?;
    let json  = response.json::<NaverResponse>().await?;
    Ok(json.response)
}


//

pub async fn github_oauth2_client() -> BasicClient {
    let o = super::config::get_config_oauth2();
    BasicClient::new(
        ClientId::new(format!("{}",o.oauth2_github_client_id)),
        Some(ClientSecret::new(format!("{}", o.oauth2_github_client_secret))),
        AuthUrl::new(format!("https://github.com/login/oauth/authorize")).unwrap(),
        Some(TokenUrl::new(format!("https://github.com/login/oauth/access_token")).unwrap())
    )
    .set_redirect_uri(RedirectUrl::new(format!("http://localhost:4000/auth/github/callback")).unwrap())
}


#[derive(Debug, Deserialize, Validate)]
pub struct GithubUserInfo {

}

pub async fn github_oauth2_user_info(access_token: &str) -> anyhow::Result<GithubUserInfo> {
    
    let url = "https://api.github.com/user";

    let client = Client::new();
    let response = client
        .get(url)
        .bearer_auth(access_token)
        .send()
        .await?;
    let json  = response.json::<serde_json::Value>().await?;
    tracing::debug!("json::: {:?}", json);
    // Ok(json.response)
    todo!()

}