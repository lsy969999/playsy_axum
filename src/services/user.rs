use sqlx::{pool::PoolConnection, PgConnection, Postgres};
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

    // add user
    let insert = repositories::user::insert_user(
            &mut tx,
            nick_name,
            email,
            &password,
            ProviderTyEnum::Email,
            UserSttEnum::WaitEmailVeri,
        ).await?;
    
    // insert wrong
    if insert.rows_affected() < 1 {
        warn!("inser user affeced 0");
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