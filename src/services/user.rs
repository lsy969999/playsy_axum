use askama::Template;
use chrono::Duration;
use sqlx::{pool::PoolConnection, types::chrono::Utc, PgConnection, Postgres};
use tracing::{error, warn};
use crate::{configs::errors::app_error::{CryptoError, ServiceLayerError, UserError}, repositories::{self, enums::{user::ProviderTyEnum, user::UserSttEnum}}, utils};

/// 회원 가입 서비스
pub async fn user_join_service(
    mut conn: PoolConnection<Postgres>,
    nick_name: &str,
    email: &str,
    password: &str,
) -> Result<(), ServiceLayerError> {
    let mut tx = repositories::tx::begin(&mut conn).await?;

    // nickname exists check
    let nick_is_some = nick_name_is_some(&mut tx, nick_name).await?;
    if nick_is_some {
        Err(UserError::NickNameExists)?;
    }

    // user exists check
    let user_is_some = user_and_ldtye_email_is_some(&mut tx, email).await?;
    if user_is_some {
        Err(UserError::UserExists)?;
    }

    // password hash
    let password = utils::hash::hash_argon2(password)
        .map_err(|error| {
            error!("passwod: {}", error);
            CryptoError::Argon2GenFail
        })?;

    let sequence = repositories::user::select_next_user_seq(&mut tx).await?;
    let user_sn = sequence.nextval as i32;

    // add user
    let _insert = repositories::user::insert_user(
            &mut tx,
            user_sn,
            nick_name,
            email,
            &password,
            ProviderTyEnum::Email,
            UserSttEnum::WaitEmailVeri,
        ).await?;
        
    // email_code_dup_chk
    for i in 1..=4 {
        let email_code = utils::rand::generate_alphanumeric_code();
        let dup_chk = repositories::email_join_verifications::select_email_join_veri_for_code_dup_chk(&mut tx, &email_code).await?;
        if dup_chk.is_none() {
            let now = Utc::now();
            let seven_days_later = now + Duration::days(7);
            let _insert = repositories::email_join_verifications::insert_email_join_veri(&mut tx, user_sn, &email_code, seven_days_later).await?;
            let result = email_join_verification_code_send(email, &email_code);
            if let Err(error) = result {
                tracing::error!("email send error! error: {error}, user_sn: {user_sn}, to: {email}, code: {email_code}");
            }
            break;
        }
        tracing::warn!("oh... email veri code is dup! user_sn: {user_sn} code: {email_code}, retry {i}");
        if i == 4 {
            Err(ServiceLayerError::CustomUser(UserError::UserExists))?
        }
    }

    repositories::tx::commit(tx).await?;
    Ok(())
}

pub async fn nick_name_is_some(conn: &mut PgConnection, nick_name: &str) -> Result<bool, ServiceLayerError> {
    let user = repositories::user::select_user_by_nick_name(
        conn,
        nick_name
    ).await?;
    Ok(user.is_some())
}

pub async fn user_and_ldtye_email_is_some(conn: &mut PgConnection, email: &str) -> Result<bool, ServiceLayerError> {
    let user = repositories::user::select_user_by_email_and_login_ty_cd(
        conn,
        email,
        ProviderTyEnum::Email,
    ).await?;
    Ok(user.is_some())
}

fn email_join_verification_code_send(to: &str, code: &str) -> anyhow::Result<()> {
    #[derive(Template)]
    #[template(path="etc/email_veri_code.html")]
    struct EmailVeriHtml<'a> {
        code: &'a str
    }

    let subject = "playsy 이메일 인증코드";
    let body = EmailVeriHtml{ code }.render()?;
    utils::mail::send_mail(to, subject, &body)?;
    Ok(())
}