use sqlx::types::chrono::{DateTime, Utc};

/// user entity
#[derive(Debug)]
pub struct User {
    pub sn: i32,
    pub nick_name: String,
    pub login_ty_cd: String,
    pub email: Option<String>,
    pub password: Option<String>,
    pub provider_id: Option<String>,
    pub user_stt_cd: String,
    pub user_ty_cd: String,
    pub created_at: DateTime<Utc>,
    pub created_by: i32,
    pub updated_at: DateTime<Utc>,
    pub updated_by: i32,
    pub is_deleted: bool,
}