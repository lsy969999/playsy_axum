use askama::Template;
use crate::models::user_info::UserInfo;

#[derive(Template)]
#[template(path="pages/board/board.html")]
pub struct BoardTemplate {
    pub user_info: Option<UserInfo>
}

#[derive(Template)]
#[template(path="pages/board/board_detail.html")]
pub struct BoardDetailTemplate {
    pub user_info: Option<UserInfo>
}

#[derive(Template)]
#[template(path="pages/board/board_edit.html")]
pub struct BoardEditTemplate {
    pub user_info: Option<UserInfo>
}
