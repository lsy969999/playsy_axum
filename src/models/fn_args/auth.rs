pub struct EmailLoginArgs {
    pub email: String,
    pub password: String,
    pub addr: String,
    pub user_agent: String
}

// pub struct GoogleLoginArgs<'a> {
//     pub provider_access_token: Option<&'a str>,
//     pub provider_refresh_token: Option<&'a str>,
//     pub info: serde_json::Value,
//     pub addr: String,
//     pub user_agent: String,
// }

// pub struct NaverLoginArgs<'a> {
//     pub provider_access_token: Option<&'a str>,
//     pub provider_refresh_token: Option<&'a str>,
//     pub info: serde_json::Value,
//     pub addr: String,
//     pub user_agent: String,
// }

// pub struct GithubLoginArgs<'a> {
//     pub provider_access_token: Option<&'a str>,
//     pub provider_refresh_token: Option<&'a str>,
//     pub info: serde_json::Value,
//     pub addr: String,
//     pub user_agent: String,
// }

#[derive(Clone)]
pub struct SocialLoginArgs<'a> {
    pub provider_access_token: Option<&'a str>,
    pub provider_refresh_token: Option<&'a str>,
    pub info: serde_json::Value,
    pub addr: String,
    pub user_agent: String,
}