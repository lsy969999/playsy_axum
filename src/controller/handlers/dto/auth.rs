use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct LoginAuthReqDto {
    #[validate(length(min = 1, message = "email len min 1"))]
    pub email: String,
    #[validate(length(min = 1, message = "password len min 1"))]
    pub password: String,
}