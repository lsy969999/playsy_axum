use sqlx::PgConnection;
use validator::{Validate, ValidationError};
use crate::{models::{request::user::{JoinEmailReqDto, MyPageUpdateReqDto}, traits::validator::AdditionalValidate}, services};
use super::generate_validation_error;

impl AdditionalValidate for JoinEmailReqDto {
    async fn additional_db_validate(&self, conn: &mut PgConnection) -> anyhow::Result<Vec<ValidationError>> {
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
            let original_val_errs = aa.field_errors().iter().map(|(_k, v)|{
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

impl AdditionalValidate for MyPageUpdateReqDto {
     async fn additional_db_validate(&self, conn: &mut PgConnection) -> anyhow::Result<Vec<ValidationError>> {
        let mut val_errs: Vec<ValidationError> = Vec::new();

        // 프로필 이미지 체크
        if let Some(pi) = &self.profile_image {
            if let Some(ct) = &pi.metadata.content_type {
                if !ct.contains("image") {
                    val_errs.push(generate_validation_error("cvf_0", "프로필은 이미지 형식만 올수 있습니다."))
                }
            }
        }

        // 닉네임 체크
        if let Some(nick) = &self.nick_name {
            let is_nick_name_some = services::user::nick_name_is_some(conn, nick).await?;
            if is_nick_name_some {
                val_errs.push(generate_validation_error("cvf_2", "이미 존재하는 닉네임 입니다."))
            }
        }

        // 원래 validate 체크
        let validate = self.validate();
        if let Err(aa) = validate {
            let original_val_errs = aa.field_errors().iter().map(|(_k, v)|{
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