use askama::Template;

use crate::models::user_info::UserInfo;

#[derive(Template)]
#[template(path="pages/chat.html")]
pub struct ChatTempalte {
    pub user_info: Option<UserInfo>
}