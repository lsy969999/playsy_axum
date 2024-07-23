use sqlx::types::chrono::{DateTime, Utc};

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

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "provider_ty_enum")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProviderTyEnum {
    Email,
    Google,
    Kakao,
    Naver,
    Github,
    Apple,
    Facebook,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "user_stt_enum")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserSttEnum {
    WaitEmailVeri,
    Ok,
    Quit
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "user_ty_enum")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserTyEnum {
    User,
    Admin
}