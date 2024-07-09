use jsonwebtoken::EncodingKey;
use sqlx::{pool::PoolConnection, types::chrono::DateTime, Acquire, Postgres};
use time::{Duration, OffsetDateTime};
use tracing::error;
use crate::{configs::{consts::DB_CODE, errors::auth::AuthError, models::auth::Claims}, repositories, utils::{self, hash::verify_argon2}};

// 로그인 요청 처리
pub async fn auth_email_request(
    mut conn: PoolConnection<Postgres>,
    email: String,
    password: String,
    jwt_acc_encoding_key: &EncodingKey,
    jwt_refr_encoding_key: &EncodingKey,
) -> Result<(String, String), AuthError> {
    let mut tx: sqlx::Transaction<Postgres> = conn.begin()
        .await
        .map_err(|error| {
            error!("auth_email_request begin error: {}", error);
            AuthError::Db
        })?;

    // 이메일과, 로그인타입코드로 유저 조회
    let user_select = repositories::user::select_user_by_email_and_login_ty_cd(
            &mut *tx,
            email,
            DB_CODE.login_ty_cd.email.to_string()
        )
        .await
        .map_err(|error| {
            error!("error {}", error);
            AuthError::Db
        })?;

    // 유저 존재 체크
    let user = match user_select {
        Some(user) => user,
        None => return Err(AuthError::UserNotExists)
    };

    // 패스워드 언랩
    let password_hash = match user.password {
        Some(pass) => pass,
        None => return Err(AuthError::UserPasswordNotExists)
    };

    // 해시 매치 검증
    let result = verify_argon2(password, password_hash)
        .map_err(|error| {
            error!("error {}", error);
            AuthError::PasswordVerify
        })?;

    // 비밀번호 틀리다면 실패
    if !result {
        return Err(AuthError::PasswordNotMatch);
    }

    // 토큰클레임 생성
    let now: OffsetDateTime = OffsetDateTime::now_utc();
    let access_claims = Claims {
        sub: user.sn.to_string(),
        exp: (now + Duration::seconds(1 * 60)).unix_timestamp() as usize,
        iat: now.unix_timestamp() as usize,
        scope: None,
    };
    let refresh_claims = Claims {
        sub: user.sn.to_string(),
        exp: (now + Duration::seconds(1 * 60 * 60)).unix_timestamp() as usize,
        iat: now.unix_timestamp() as usize,
        scope: None,
    };

    // 토큰스트링 생성
    let access_token = utils::jwt::generate_jwt(&access_claims, jwt_acc_encoding_key)
        .map_err(|error| {
            error!("error {}", error);
            AuthError::TokenCreation
        })?;
    let refresh_token = utils::jwt::generate_jwt(&refresh_claims, jwt_refr_encoding_key)
        .map_err(|error| {
            error!("error {}", error);
            AuthError::TokenCreation
        })?;

    // 리프래시토큰 디비 저장값 생성
    let refresh_token_hash = utils::hash::hash_sha_256(&refresh_token);
    let db_refr_exp_timestap = match DateTime::from_timestamp(now.unix_timestamp(), 0) {
        Some(time) => time,
        None => return Err(AuthError::TokenCreation),
    };
    
    // 리프래시 토큰 정보 저장
    repositories::refresh_token::insert_refresh_token(&mut *tx, user.sn, refresh_token_hash, refresh_token.clone(), db_refr_exp_timestap)
        .await
        .map_err(|error| {
            error!("error {}", error);
            AuthError::Db
        })?;

    tx.commit()
        .await
        .map_err(|error| {
            error!("auth_email_request commit error: {}", error);
            AuthError::Db
        })?;
    Ok((access_token, refresh_token))
}

