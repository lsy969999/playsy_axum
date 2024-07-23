use sqlx::types::chrono::{DateTime, Utc};

use crate::repositories::enums::{user::ProviderTyEnum, user::UserSttEnum, user::UserTyEnum};

/// user entity
#[derive(Debug)]
pub struct User {
    pub sn: i32,
    pub nick_name: String,
    pub avatar_url: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub user_stt_enum: UserSttEnum,
    pub user_ty_enum: UserTyEnum,
    pub provider_ty_enum: ProviderTyEnum,
    pub provider_id: String,
    pub provider_access_token: Option<String>,
    pub provider_refresh_token: Option<String>,
    pub provider_etc: Option<serde_json::Value>,
    //
    pub created_at: DateTime<Utc>,
    pub created_by: i32,
    pub updated_at: DateTime<Utc>,
    pub updated_by: i32,
    pub is_deleted: bool,
}