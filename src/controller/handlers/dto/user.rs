use serde::Deserialize;
use validator::Validate;
use crate::configs::validator::{pass_vali_1_lower, pass_vali_1_num, pass_vali_1_upper, pass_vali_len_8, pass_vali_special_char, nick_name_vali_dup_chk};

#[derive(Deserialize, Debug, Validate)]
#[validate(context = "bool")]
pub struct JoinReqDto {
    #[validate(length(min = 3, message = "닉네임은 3글자 이상 이어야 합니다."))]
    #[validate(custom(function="nick_name_vali_dup_chk", use_context))]
    pub nick_name: String,

    #[validate(email(message="이메일 형식 이어야 합니다."))]
    pub email: String,
    
    #[validate(custom(function="pass_vali_len_8"))]
    #[validate(custom(function="pass_vali_1_upper"))]
    #[validate(custom(function="pass_vali_1_lower"))]
    #[validate(custom(function="pass_vali_1_num"))]
    #[validate(custom(function="pass_vali_special_char"))]
    pub password: String,
}
