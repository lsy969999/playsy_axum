use sqlx::{pool::PoolConnection, types::chrono::DateTime, Postgres};
use time::{Duration, OffsetDateTime};
use tracing::error;
use crate::{configs::{errors::app_error::{AuthError, CryptoError, ServiceLayerError, UserError}, models::claims::Claims}, repositories::{self, enums::user::ProviderTyEnum}, utils::{self}};

// 로그인 요청 처리
pub async fn auth_email_request(
    mut conn: PoolConnection<Postgres>,
    email: &str,
    password: &str,
) -> Result<(String, String), ServiceLayerError> {
    let mut tx = repositories::tx::begin(&mut conn).await?;

    // 이메일과, 로그인타입코드로 유저 조회
    let user_select = repositories::user::select_user_by_email_and_login_ty_cd(
            &mut tx,
            email,
            ProviderTyEnum::Email
        ).await?;

    // 유저 존재 체크
    let user = match user_select {
        Some(user) => user,
        None => return Err(UserError::UserNotExists)?
    };

    // 패스워드 언랩
    let password_hash = match user.password {
        Some(pass) => pass,
        None => return Err(UserError::UserPasswordNotExists)?
    };

    // 해시 매치 검증
    let result = utils::hash::verify_argon2(password, &password_hash)
        .map_err(|error| {
            error!("error {}", error);
            CryptoError::Argon2VerfyFail
        })?;

    // 비밀번호 틀리다면 실패
    if !result {
        return Err(UserError::UserPasswordNotMatch)?;
    }

    // 토큰클레임 생성
    let now: OffsetDateTime = OffsetDateTime::now_utc();
    let acc_exp = *utils::settings::get_jwt_access_time();
    let refr_exp = *utils::settings::get_jwt_refresh_time();
    let access_claims = Claims::new(user.sn.to_string(), now + Duration::seconds(acc_exp), now, None);
    let refresh_claims = Claims::new(user.sn.to_string(), now + Duration::seconds(refr_exp), now, None);

    // 토큰 생성
    let acc = utils::settings::get_settings_jwt_access_keys();
    let access_token = utils::jwt::generate_jwt(&access_claims, &acc.encoding)?;
    let refr = utils::settings::get_settings_jwt_refresh_keys();
    let refresh_token = utils::jwt::generate_jwt(&refresh_claims, &refr.encoding)?;

    // 리프래시토큰 디비 저장값 생성
    let refresh_token_hash = utils::hash::hash_sha_256(&refresh_token);
    let db_refr_exp_timestap = match DateTime::from_timestamp(now.unix_timestamp(), 0) {
        Some(time) => time,
        None => return Err(AuthError::TokenCreation)?,
    };
    
    // 리프래시 토큰 정보 저장
    repositories::refresh_token::insert_refresh_token(
            &mut tx,
            user.sn,
            refresh_token_hash,
            refresh_token.clone(),
            db_refr_exp_timestap
        ).await?;

    repositories::tx::commit(tx).await?;
    Ok((access_token, refresh_token))
}
