use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Inquery {
    pub sn: i32,
    pub user_sn: Option<i32>,
    pub email: Option<String>,
    pub subject: Option<String>,
    pub message: Option<String>,
    pub answered_at: Option<DateTime<Utc>>,
    pub answer: Option<String>,
    //
    pub created_at: DateTime<Utc>,
    pub created_by: i32,
    pub updated_at: DateTime<Utc>,
    pub updated_by: i32,
    pub is_deleted: bool,
}