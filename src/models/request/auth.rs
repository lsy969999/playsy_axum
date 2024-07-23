use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct LoginAuthReqDto {
    pub authenticity_token: String,
    pub email: String,
    pub password: String,
}