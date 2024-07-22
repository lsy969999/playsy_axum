use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, Scope, TokenUrl};
use reqwest::Client;
use serde::Deserialize;

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

#[derive(Debug, Deserialize)]
pub struct GoogleOauth2UserInfo {
    pub sub: Option<String>,
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub picture: Option<String>,
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