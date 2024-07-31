use askama::Template;
use crate::models::user_info::UserInfo;

#[derive(Template)]
#[template(path="pages/about.html")]
pub struct AboutTemplate {
    pub user_info: Option<UserInfo>
}