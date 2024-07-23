use askama::Template;

use crate::models::user_info::UserInfo;

#[derive(Template)]
#[template(path="pages/home.html")]
pub struct HomeTemplate {
    pub user_info: Option<UserInfo>
}

#[derive(Template)]
#[template(path="pages/privacy.html")]
pub struct PrivacyTemplate {
    pub user_info: Option<UserInfo>
}