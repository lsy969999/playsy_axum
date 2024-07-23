use askama::Template;
use crate::configs::models::user_info::UserInfo;

#[derive(Template)]
#[template(path="pages/chat.html")]
pub struct ChatTempalte {
    pub user_info: Option<UserInfo>
}