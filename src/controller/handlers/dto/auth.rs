use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct LoginAuthReqDto {
    pub email: String,
    pub password: String,
}