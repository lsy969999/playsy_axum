use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::validators::nick_name_vali_char;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct NaverOauth2UserInfo {
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    #[validate(length(min = 3, max=10, message = "닉네임은 3글자 이상 10글자 미만 이어야 합니다."))]
    #[validate(custom(function="nick_name_vali_char"))]
    pub nickname: Option<String>,
    pub profile_image: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NaverResponse {
    pub resultcode: String,
    pub message: String,
    pub response: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GoogleOauth2UserInfo {
    pub sub: String,
    #[validate(length(min = 3, max=10, message = "닉네임은 3글자 이상 10글자 미만 이어야 합니다."))]
    #[validate(custom(function="nick_name_vali_char"))]
    pub name: Option<String>,
    pub picture: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct GithubOauth2UserInfo {

}