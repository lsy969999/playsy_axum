use sqlx::{pool::PoolConnection, Acquire, Postgres};
use tracing::{error, warn};
use crate::{configs::errors::app_error::{CryptoError, ServiceLayerError, UserError}, repositories, utils};

/// 회원 가입 서비스
pub async fn user_join_service(
    mut conn: PoolConnection<Postgres>,
    nick_name: String,
    email: String,
    password: String,
) -> Result<(), ServiceLayerError> {
    let mut tx = conn.begin().await
        .map_err(|e| ServiceLayerError::DbTx(e))?;

    // nickname check
    let user = repositories::user::select_user_by_nick_name(
            &mut tx,
            nick_name.clone()
        ).await?;

    // nickname duplicate
    if user.is_some() {
        return Err(UserError::NickNameExists)?;
    }

    // pass hash
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
            password
        ).await?;
    
    // insert wrong
    if insert.rows_affected() < 1 {
        warn!("inser user affeced 0");
    }

    tx.commit().await
        .map_err(|e| ServiceLayerError::DbTx(e))?;

    Ok(())
}