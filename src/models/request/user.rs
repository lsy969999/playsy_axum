use std::borrow::Cow;

use axum_typed_multipart::{FieldData, TryFromMultipart};
use serde::Deserialize;
use sqlx::PgConnection;
use validator::{Validate, ValidationError};

use crate::{configs::errors::app_error::PageHandlerLayerError, services, validators::{email_vali_dup_chk, nick_name_vali_char, nick_name_vali_dup_chk, pass_vali_1_lower, pass_vali_1_num, pass_vali_1_upper, pass_vali_password_confirmation, pass_vali_special_char, JoinReqValiContext }};

#[derive(Deserialize, Debug, Validate)]
#[validate(context = "JoinReqValiContext")]
pub struct JoinNickNameReqDto {
    #[validate(length(min = 3, max=10, message = "닉네임은 3글자 이상 10글자 미만 이어야 합니다."))]
    #[validate(custom(function="nick_name_vali_dup_chk", use_context))]
    #[validate(custom(function="nick_name_vali_char"))]
    pub nick_name: String,
}

#[derive(Deserialize, Debug, Validate)]
#[validate(context = "JoinReqValiContext")]
pub struct EmailValidateReqDto {
    #[validate(email(message="이메일 형식 이어야 합니다."))]
    #[validate(custom(function="email_vali_dup_chk", use_context))]
    pub email: String,
}

#[derive(Deserialize, Debug, Validate)]
#[validate(context = "JoinReqValiContext")]
pub struct JoinReqDto {
    #[validate(length(min = 3, max=10, message = "닉네임은 3글자 이상 10글자 미만 이어야 합니다."))]
    #[validate(custom(function="nick_name_vali_dup_chk", use_context))]
    #[validate(custom(function="nick_name_vali_char"))]
    pub nick_name: String,

    #[validate(email(message="이메일 형식 이어야 합니다."))]
    #[validate(custom(function="email_vali_dup_chk", use_context))]
    pub email: String,
    
    #[validate(length(min = 8, max=50, message = "비밀번호는 8자 이상이어야 합니다."))]
    #[validate(custom(function="pass_vali_1_upper"))]
    #[validate(custom(function="pass_vali_1_lower"))]
    #[validate(custom(function="pass_vali_1_num"))]
    #[validate(custom(function="pass_vali_special_char"))]
    pub password: String,
}

#[derive(Deserialize, Debug, Validate)]
#[validate(context = "JoinReqValiContext")]
pub struct NickNameUpdateDto {
    #[validate(length(min = 3, max=10, message = "닉네임은 3글자 이상 10글자 미만 이어야 합니다."))]
    #[validate(custom(function="nick_name_vali_dup_chk", use_context))]
    #[validate(custom(function="nick_name_vali_char"))]
    pub nick_name: String,
}

#[derive(TryFromMultipart, Debug, Validate)]
// #[validate(context = "JoinReqValiContext")]
pub struct JoinEmailReqDto {
    pub profile_image: Option<FieldData<axum::body::Bytes>>,

    #[validate(length(min = 3, max=10, message = "닉네임은 3글자 이상 10글자 미만 이어야 합니다."))]
    // #[validate(custom(function="nick_name_vali_dup_chk", use_context))]
    #[validate(custom(function="nick_name_vali_char"))]
    pub nick_name: String,

    #[validate(email(message="이메일 형식 이어야 합니다."))]
    // #[validate(custom(function="email_vali_dup_chk", use_context))]
    pub email: String,
    
    #[validate(length(min = 8, max=50, message = "비밀번호는 8자 이상이어야 합니다."))]
    #[validate(custom(function="pass_vali_1_upper"))]
    #[validate(custom(function="pass_vali_1_lower"))]
    #[validate(custom(function="pass_vali_1_num"))]
    #[validate(custom(function="pass_vali_special_char"))]
    pub password: String,

    // #[validate(custom(function="pass_vali_password_confirmation", use_context))]
    pub password_confirmation: String,
}

impl JoinEmailReqDto {
    pub async fn additional_validate(&self, conn: &mut PgConnection) -> Result<Vec<ValidationError>, PageHandlerLayerError> {
        let mut val_errs: Vec<ValidationError> = Vec::new();
        //추가적인 validate

        // 프로필 이미지 체크
        if let Some(pi) = &self.profile_image {
            if let Some(ct) = &pi.metadata.content_type {
                if !ct.contains("image") {
                    val_errs.push(generate_validation_error("cvf_0", "프로필은 이미지 형식만 올수 있습니다."))
                }
            }
        }

        // 이메일 체크
        let is_email_some = services::user::user_and_ldtye_email_is_some(conn, &self.email).await?;
        if is_email_some {
            val_errs.push(generate_validation_error("cvf_1", "이미 존재하는 이메일 입니다."))
        }
        
        // 닉네임 체크
        let is_nick_name_some = services::user::nick_name_is_some(conn, &self.nick_name).await?;
        if is_nick_name_some {
            val_errs.push(generate_validation_error("cvf_2", "이미 존재하는 닉네임 입니다."))
        }

        // 비밀번호 일치성 체크
        if self.password != self.password_confirmation {
            val_errs.push(generate_validation_error("cvf_4", "비밀번호와 비밀번호 확인이 일치하지 않습니다."))
        }

        // 원래 validate 체크
        let validate = self.validate();
        if let Err(aa) = validate {
            let original_val_errs = aa.field_errors().iter().map(|(k, v)|{
                v.iter().map(|ve|{
                    ve.to_owned()
                })
                .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();
            val_errs.extend(original_val_errs)
        }

        Ok(val_errs)
    }
}

fn generate_validation_error(code: &'static str, message: &'static str) -> ValidationError {
    ValidationError::new(code).with_message(Cow::Borrowed(message))
} 