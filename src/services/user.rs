use sqlx::{pool::PoolConnection, Acquire, Postgres};
use tracing::error;

use crate::{configs::errors::user_join::UserJoinError, repositories, utils};

/// 회원 가입 서비스
pub async fn user_join_service(
    mut conn: PoolConnection<Postgres>,
    nick_name: String,
    email: String,
    password: String,
) -> Result<(), UserJoinError> {
    let mut tx = conn.begin()
        .await
        .map_err(|error| {
            error!("user_join_service begin error: {}", error);
            UserJoinError::Db
        })?;
    
    // nickname check
    let user = repositories::user::select_user_by_nick_name(
            &mut *tx,
            nick_name.clone()
        )
        .await
        .map_err(|error| {
            error!("user_join_service error: {}", error);
            UserJoinError::Db
        })?;

    // nickname duplicate
    if user.is_some() {
        return Err(UserJoinError::NickNameExists);
    }

    // pass hash
    let password = utils::hash::hash_argon2(password)
        .map_err(|error| {
            error!("user_join_service error: {}", error);
            UserJoinError::PassGenFail
        })?;

    // add user
    let insert = repositories::user::insert_user(
            &mut *tx,
            nick_name,
            email,
            password
        )
        .await
        .map_err(|error| {
            error!("user_join_service error: {}", error);
            UserJoinError::Db
        })?;
    
    // insert wrong
    if insert.rows_affected() < 1 {
        return Err(UserJoinError::InsertFail);
    }

    tx.commit()
        .await
        .map_err(|error| {
            error!("user_join_service commit error: {}", error);
            UserJoinError::Db
        })?;

    Ok(())
}