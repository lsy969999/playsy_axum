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