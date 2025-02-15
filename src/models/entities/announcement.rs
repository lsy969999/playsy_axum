use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Announcement {
    pub sn: i32,
    pub user_sn: i32,
    pub title: String,
    pub content: String,
    //
    pub created_at: DateTime<Utc>,
    pub created_by: i32,
    pub updated_at: DateTime<Utc>,
    pub updated_by: i32,
    pub is_deleted: bool,
}