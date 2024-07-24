use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::validators::nick_name_vali_char;

use super::traits::oauth2::SocaliLoginValidateProcess;

#[derive(Debug, Deserialize, Validate)]
pub struct NaverOaut2Response {
    pub resultcode: String,
    pub message: String,
    pub response: serde_json::Value,
}

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

impl SocaliLoginValidateProcess for NaverOauth2UserInfo {
    fn get_id(&self) -> String {
        self.id.clone()
    }
    fn get_avatar_url(&self) -> Option<String> {
        self.profile_image.clone()
    }
    fn get_email(&self) -> Option<String> {
        self.email.clone()
    }
    fn get_nick_name(&self) -> Option<String> {
        self.email.clone()
    }
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

impl SocaliLoginValidateProcess for GoogleOauth2UserInfo {
    fn get_id(&self) -> String {
        self.sub.clone()
    }
    fn get_nick_name(&self) -> Option<String> {
        self.name.clone()
    }
    fn get_email(&self) -> Option<String> {
        self.email.clone()
    }
    fn get_avatar_url(&self) -> Option<String> {
        self.picture.clone()
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct GithubOauth2UserInfo {
    pub id: u32,
    pub avatar_url: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>
}

impl SocaliLoginValidateProcess for GithubOauth2UserInfo {
    fn get_id(&self) -> String {
        self.id.to_string()
    }
    fn get_avatar_url(&self) -> Option<String> {
        self.avatar_url.clone()
    }
    fn get_email(&self) -> Option<String> {
        self.email.clone()
    }
    fn get_nick_name(&self) -> Option<String> {
        self.name.clone()
    }
}