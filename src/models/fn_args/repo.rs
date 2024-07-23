use chrono::{DateTime, Utc};

pub struct InsertRefreshTokenArgs<'a> {
    pub sn: i32,
    pub user_sn: i32,
    pub hash: &'a str,
    pub refresh_token: &'a str,
    pub expires_at: DateTime<Utc>,
    pub forwarded_id: Option<&'a str>,
    pub addr: &'a str,
    pub user_agent: &'a str,
}