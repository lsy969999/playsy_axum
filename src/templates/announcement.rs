use askama::Template;
use crate::templates::filters;
use crate::models::{entities::announcement::Announcement, response::pagination::PaginationRes, user_info::UserInfo};

#[derive(Template)]
#[template(path="pages/announcement/announcement.html")]
pub struct AnnouncementTemplate {
    pub user_info: Option<UserInfo>,
    pub announcements: Vec<Announcement>,
    pub pagination: PaginationRes,
}

#[derive(Template)]
#[template(path="pages/announcement/announcement_detail.html")]
pub struct AnnouncementDetailTemplate {
    pub user_info: Option<UserInfo>,
    pub announcement: Announcement,
}