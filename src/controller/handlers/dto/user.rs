use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct JoinReqDto {
    #[validate(length(min = 1, message = "email len min 1"))]
    pub email: String,
    #[validate(length(min = 1, message = "password len min 1"))]
    pub password: String,
    #[validate(length(min = 1, message = "nick_name len min 1"))]
    pub nick_name: String,
}