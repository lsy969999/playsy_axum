use chrono::{DateTime, Utc};

use crate::models::entities::user::{ProviderTyEnum, UserSttEnum};

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

pub struct InsertUserArgs<'a> {
    pub avatar_url: Option<&'a str>,
    pub user_sn: i32,
    pub nick_name: &'a str,
    pub email: Option<&'a str>,
    pub password: Option<&'a str>,
    pub provider_ty_enum: ProviderTyEnum,
    pub provider_id: &'a str,
    pub provider_access_token: Option<&'a str>,
    pub provider_refresh_token: Option<&'a str>,
    pub provider_etc: Option<serde_json::Value>,
    pub user_stt_enum: UserSttEnum,
}