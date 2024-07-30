use askama::Template;
use chrono::Duration;
use sqlx::{types::chrono::Utc, PgConnection};
use crate::{configs::errors::app_error::{CryptoError, ServiceLayerError, UserError}, models::{entities::user::{ProviderTyEnum, User, UserSttEnum}, fn_args::repo::{self, InsertUserArgs}}, repositories::{self}, utils};

/// 회원 가입 서비스
pub async fn user_join_service(
    conn: &mut PgConnection,
    nick_name: &str,
    email: &str,
    password: &str,
) -> Result<User, ServiceLayerError> {
    let mut tx = repositories::tx::begin(conn).await?;

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
            tracing::error!("passwod: {}", error);
            CryptoError::Argon2GenFail
        })?;

    let sequence = repositories::user::select_next_user_seq(&mut tx).await?;
    let user_sn = sequence.nextval as i32;

    let email_provider_id = utils::hash::hash_sha_256(&format!("{}:{}", user_sn, email));
    
    // add user
    let user = repositories::user::insert_user(
            &mut tx,
            InsertUserArgs {
                avatar_url: None,
                email: Some(email),
                nick_name,
                password: Some(&password),
                provider_access_token: None,
                provider_refresh_token: None,
                provider_id: &email_provider_id,
                provider_etc: None,
                provider_ty_enum: ProviderTyEnum::Email,
                user_stt_enum: UserSttEnum::WaitEmailVeri,
                user_sn
            }
        ).await?;
        
        email_verification_code_generate_and_mail_send(&mut tx, email, user_sn).await?;

    repositories::tx::commit(tx).await?;
    Ok(user)
}

pub async fn nick_name_is_some(conn: &mut PgConnection, nick_name: &str) -> Result<bool, ServiceLayerError> {
    let user = repositories::user::select_user_by_nick_name(
        conn,
        nick_name
    ).await?;
    Ok(user.is_some())
}

pub async fn user_by_email_and_provider_ty_enum_is_some(conn: &mut PgConnection, email: &str, provider_type: ProviderTyEnum) -> Result<bool, ServiceLayerError> {
    let user = repositories::user::select_user_by_email_and_login_ty_cd(
        conn,
        email,
        provider_type,
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

async fn email_join_verification_code_send(to: &str, code: &str) -> anyhow::Result<()> {
    #[derive(Template)]
    #[template(path="etc/email_veri_code.html")]
    struct EmailVeriHtml<'a> {
        code: &'a str
    }

    let subject = "playsy 이메일 인증코드";
    let body = EmailVeriHtml{ code: code }.render()?;
    utils::mail::send_mail(to, subject, &body).await?;
    Ok(())
}


pub async fn select_user(conn: &mut PgConnection, sn: i32) -> Result<User, ServiceLayerError> {
    let useroption = repositories::user::select_user_by_sn(conn, sn).await?;
    Ok(
        match useroption {
            Some(user) => user,
            None => Err(UserError::UserNotExists)?
        }
    )
}

// pub async fn mypage_update(
//     conn: &mut PgConnection,
//     user_sn: i32,
//     nick_name: Option<&str>,
//     avatar_url: Option<&str>,
// ) -> Result<(), ServiceLayerError> {
//     let mut tx = repositories::tx::begin(conn).await?;

//     if let Some(nick) = nick_name {
//         repositories::user::update_user_nick_name_by_sn(&mut tx, user_sn, nick).await?;
//     }

//     if let Some(avatar) = avatar_url {
//         repositories::user::update_user_avatar_url_by_sn(&mut tx, user_sn, avatar).await?;
//     }

//     repositories::tx::commit(tx).await?;
//     Ok(())
// }

pub async fn user_withdrawl(conn: &mut PgConnection, sn: i32) -> Result<u64, ServiceLayerError> {
    Ok(repositories::user::delete_user_by_sn(conn, sn).await?.rows_affected())
}

pub async fn update_user_nick_name(conn: &mut PgConnection, user_sn: i32, nick_name: &str) -> Result<(), ServiceLayerError> {
    repositories::user::update_user_nick_name_by_sn(conn, user_sn, nick_name).await?;
    Ok(())
}

pub async fn update_user_avatar_url(conn: &mut PgConnection, user_sn: i32, avatar_url: &str) -> Result<(), ServiceLayerError> {
    repositories::user::update_user_avatar_url_by_sn(conn, user_sn, avatar_url).await?;
    Ok(())
}

pub async fn email_verification(conn: &mut PgConnection, user_sn: i32, code: &str) ->Result<bool, ServiceLayerError> {
    let mut tx = repositories::tx::begin(conn).await?;
    let email: Option<crate::models::entities::email_join_verifications::EmailJoinVerifications> = repositories::email_join_verifications::select_email_join_veri_by_user_sn_and_code(&mut tx, user_sn, code).await?;
    if email.is_none() {
        return Ok(false);
    }
    repositories::email_join_verifications::update_email_join_veri_is_verified_to_true_by_sn_and_code(&mut tx, user_sn, code).await?;
    repositories::user::update_user_stt_enum(&mut tx, user_sn, UserSttEnum::Ok).await?;
    repositories::tx::commit(tx).await?;
    Ok(true)
}

pub async fn email_verification_resend(conn: &mut PgConnection, user_sn: i32) -> Result<(), ServiceLayerError> {
    let mut tx = repositories::tx::begin(conn).await?;
    let user = repositories::user::select_user_by_sn(&mut tx, user_sn).await?;
    match user {
        Some(user) => {
            let email = match user.email {
                Some(email) => email,
                None => Err(UserError::UserNotExists)?
            };
            repositories::email_join_verifications::delete_email_join_veri_by_user_sn(&mut tx, user_sn).await?;
            email_verification_code_generate_and_mail_send(&mut tx, &email, user_sn).await?
        }
        None => Err(UserError::UserNotExists)?
    }
    repositories::tx::commit(tx).await?;
    Ok(())
}


async fn email_verification_code_generate_and_mail_send(tx: &mut PgConnection, email: &str, user_sn: i32,) -> Result<(), ServiceLayerError> {
    // email_code_dup_chk
    for i in 1..=4 {
        let email_code = utils::rand::generate_alphanumeric_code(10);
        let dup_chk = repositories::email_join_verifications::select_email_join_veri_for_code_dup_chk(tx, &email_code).await?;
        if dup_chk.is_none() {
            let now = Utc::now();
            let seven_days_later = now + Duration::days(7);
            let _insert = repositories::email_join_verifications::insert_email_join_veri(tx, user_sn, &email_code, seven_days_later).await?;
            let email = email.to_string();
            tokio::spawn(async move {
                let user_sn = user_sn.clone();
                let email = email.clone();
                let email_code = email_code.clone();
                let result = email_join_verification_code_send(&email, &email_code).await;
                if let Err(error) = result {
                    tracing::error!("email send error! error: {error}, user_sn: {user_sn}, to: {email}, code: {email_code}");
                }
            });
            break;
        }
        tracing::warn!("oh... email veri code is dup! user_sn: {user_sn} code: {email_code}, retry {i}");
    }
    Ok(())
}