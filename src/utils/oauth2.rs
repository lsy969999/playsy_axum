use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use reqwest::Client;
use crate::{configs::etc::oauth2_naver_client::NaverClient, models::oauth2::NaverOaut2Response};

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

pub async fn google_oauth2_user_info_api(access_token: &str) -> anyhow::Result<serde_json::Value> {
    let url = "https://www.googleapis.com/oauth2/v3/userinfo";
    let client = Client::new();
    let response = client
        .get(url)
        .bearer_auth(access_token)
        .send()
        .await?;
    Ok(response.json::<serde_json::Value>().await?)
}

pub fn naver_oauth2_client() -> NaverClient {
    let o = super::config::get_config_oauth2();
    NaverClient::new(
        ClientId::new(format!("{}",o.oauth2_naver_client_id)),
        Some(ClientSecret::new(format!("{}", o.oauth2_naver_client_secret))),
        AuthUrl::new(format!("https://nid.naver.com/oauth2.0/authorize")).unwrap(),
        Some(TokenUrl::new(format!("https://nid.naver.com/oauth2.0/token")).unwrap())
    )
    .set_redirect_uri(RedirectUrl::new(format!("http://localhost:4000/auth/naver/callback")).unwrap())
}

pub async fn naver_oauth2_user_info_api(access_token: &str) -> anyhow::Result<serde_json::Value> {
    let url = "https://openapi.naver.com/v1/nid/me";
    let client = Client::new();
    let response = client
        .get(url)
        .bearer_auth(access_token)
        .send()
        .await?;
    let json  = response.json::<NaverOaut2Response>().await?;
    Ok(serde_json::to_value(json.response)?)
}

pub fn github_oauth2_client() -> BasicClient {
    let o = super::config::get_config_oauth2();
    BasicClient::new(
        ClientId::new(format!("{}",o.oauth2_github_client_id)),
        Some(ClientSecret::new(format!("{}", o.oauth2_github_client_secret))),
        AuthUrl::new(format!("https://github.com/login/oauth/authorize")).unwrap(),
        Some(TokenUrl::new(format!("https://github.com/login/oauth/access_token")).unwrap())
    )
    .set_redirect_uri(RedirectUrl::new(format!("http://localhost:4000/auth/github/callback")).unwrap())
}

pub async fn github_oauth2_user_info(access_token: &str) -> anyhow::Result<serde_json::Value> {
    let url = "https://api.github.com/user";
    let client = Client::new();
    let response = client
        .get(url)
        // issue
        // user-agent 없으면 403 리턴됨 그래서 아무값 넣음
        // https://docs.github.com/en/rest/using-the-rest-api/troubleshooting-the-rest-api?apiVersion=2022-11-28#user-agent-required
        .header("User-Agent", "playsy-reqwest")
        .bearer_auth(access_token)
        .send()
        .await?;
    Ok(response.json::<serde_json::Value>().await?)
}

pub fn kakao_oauth2_client() -> BasicClient {
    let o = super::config::get_config_oauth2();
    BasicClient::new(
        ClientId::new(format!("{}",o.oauth2_kakao_client_id)),
        Some(ClientSecret::new(format!("{}", o.oauth2_kakao_client_secret))),
        AuthUrl::new(format!("https://kauth.kakao.com/oauth/authorize")).unwrap(),
        Some(TokenUrl::new(format!("https://kauth.kakao.com/oauth/token")).unwrap())
    )
    .set_redirect_uri(RedirectUrl::new(format!("http://localhost:4000/auth/kakao/callback")).unwrap())
}

pub async fn kakao_oauth2_user_info(access_token: &str) -> anyhow::Result<serde_json::Value> {
    let url = "https://kapi.kakao.com/v2/user/me";
    let client = Client::new();
    let response = client
        .get(url)
        .bearer_auth(access_token)
        .send()
        .await?;
    Ok(response.json::<serde_json::Value>().await?)
}