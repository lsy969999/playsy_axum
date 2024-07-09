use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct RefreshToken {
    pub sn: i32,
    pub user_sn: i32,
    pub hash: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub created_by: i32,
    pub updated_at: DateTime<Utc>,
    pub updated_by: i32,
    pub is_deleted: bool,
}