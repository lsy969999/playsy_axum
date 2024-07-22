use sqlx::{pool::PoolConnection, types::chrono::DateTime, Postgres};
use time::{Duration, OffsetDateTime};
use tracing::error;
use validator::Validate;
use crate::{configs::{errors::app_error::{AuthError, CryptoError, ServiceLayerError, UserError}, models::claims::{AccessClaims, RefreshClaims}}, repositories::{self, enums::user::{ProviderTyEnum, UserSttEnum}}, utils::{self, oauth2::{GoogleOauth2UserInfo, NaverUserInfo}}};

// 이메일 로그인 요청 처리
pub async fn auth_email_request(
    mut conn: PoolConnection<Postgres>,
    email: &str,
    password: &str,
    addr: String,
    user_agent: String,
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

    let sequence = repositories::refresh_token::select_next_refresh_token_seq(&mut tx).await?;
    let chk = sequence.nextval;

    // 토큰클레임 생성
    let now: OffsetDateTime = OffsetDateTime::now_utc();
    let acc_exp = *utils::config::get_config_jwt_access_time();
    let refr_exp = *utils::config::get_config_jwt_refresh_time();
    let access_claims = AccessClaims::new(user.sn.to_string(), now + Duration::seconds(acc_exp), now, None, user.nick_name, user.avatar_url);
    let refresh_claims = RefreshClaims::new(user.sn.to_string(), now + Duration::seconds(refr_exp), now, None, chk as usize);

    // 토큰 생성
    let acc = utils::config::get_config_jwt_access_keys();
    let access_token = utils::jwt::generate_jwt(&access_claims, &acc.encoding)?;
    let refr = utils::config::get_config_jwt_refresh_keys();
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
            chk as i32,
            user.sn,
            refresh_token_hash,
            refresh_token.clone(),
            db_refr_exp_timestap,
            None,
            addr,
            user_agent,
        ).await?;

    repositories::tx::commit(tx).await?;
    Ok((access_token, refresh_token))
}

// 구글 소셜 로그인 처리
/*
 1. 가입 여부 체크
   가입 안되어 있으면 가입시켜주고 토큰발급 진행

   가입되어 있으면 토큰 발급 진행
*/
pub async fn auth_google_request(
    mut conn: PoolConnection<Postgres>,
    info: GoogleOauth2UserInfo,
    provider_access_token: Option<&str>,
    addr: String,
    user_agent: String,
) -> Result<(String, String), ServiceLayerError> {
    if info.email.is_none() {
        Err(UserError::UserError)?
    }
    let mut tx = repositories::tx::begin(&mut conn).await?;
    let user_select = repositories::user::select_user_by_email_and_login_ty_cd(
        &mut tx,
        info.email.clone().unwrap().as_str(),
        ProviderTyEnum::Google
    ).await?;
    

    // 미가입 상태는 가입시켜준다.
    let (user_sn, nick_name, avatar_url) = match user_select {
        Some(user) => (user.sn, user.nick_name, user.avatar_url),
        None => {
            let mut nick_name = info.name.clone().unwrap();
            let is_nick_error = info.validate().is_err();
            if is_nick_error {
                let rand_alpha = utils::rand::generate_alphanumeric_code(4);
                nick_name = format!("User_{rand_alpha}");
            }
            
            // 가입되어 있지 않은 상태, 가입처리
            for i in 0..=4 {
                let nick_is_some = super::user::nick_name_is_some(&mut tx, &nick_name).await?;
                if nick_is_some {
                    // 닉네임 변경 필요
                    let rand_alpha = utils::rand::generate_alphanumeric_code(4);
                    nick_name = format!("User_{rand_alpha}");
                } else {
                    break
                }
                if i == 4 {
                    Err(UserError::NickNameExists)?;
                }
            }

            let sequence = repositories::user::select_next_user_seq(&mut tx).await?;
            let user_sn = sequence.nextval as i32;

            let _insert = repositories::user::insert_user(
                &mut tx,
                info.picture.as_deref(),
                user_sn,
                &nick_name,
                &info.email.unwrap(),
                None,
                ProviderTyEnum::Google,
                info.sub.as_deref(),
                None,
                provider_access_token,
                None,
                UserSttEnum::Ok,
            ).await?;
            (user_sn, nick_name, info.picture)
        }
    };

    // 로그인 처리

    let sequence = repositories::refresh_token::select_next_refresh_token_seq(&mut tx).await?;
    let chk = sequence.nextval;

    // 토큰클레임 생성
    let now: OffsetDateTime = OffsetDateTime::now_utc();
    let acc_exp = *utils::config::get_config_jwt_access_time();
    let refr_exp = *utils::config::get_config_jwt_refresh_time();
    let access_claims = AccessClaims::new(user_sn.to_string(), now + Duration::seconds(acc_exp), now, None, nick_name, avatar_url);
    let refresh_claims = RefreshClaims::new(user_sn.to_string(), now + Duration::seconds(refr_exp), now, None, chk as usize);

    // 토큰 생성
    let acc = utils::config::get_config_jwt_access_keys();
    let access_token = utils::jwt::generate_jwt(&access_claims, &acc.encoding)?;
    let refr = utils::config::get_config_jwt_refresh_keys();
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
            chk as i32,
            user_sn,
            refresh_token_hash,
            refresh_token.clone(),
            db_refr_exp_timestap,
            None,
            addr,
            user_agent,
        ).await?;

    repositories::tx::commit(tx).await?;
    Ok((access_token, refresh_token))
}

pub async fn auth_naver_request(
    mut conn: PoolConnection<Postgres>,
    info: NaverUserInfo,
    provider_access_token: Option<&str>,
    provider_refresh_token: Option<&str>,
    addr: String,
    user_agent: String,
) -> Result<(String, String), ServiceLayerError> {
    // if info.email.is_none() {
    //     Err(UserError::UserError)?
    // }
    let mut tx = repositories::tx::begin(&mut conn).await?;
    let user_select = repositories::user::select_user_by_email_and_login_ty_cd(
        &mut tx,
        info.email.clone().as_str(),
        ProviderTyEnum::Naver
    ).await?;
    

    // 미가입 상태는 가입시켜준다.
    let (user_sn, nick_name, avatar_url) = match user_select {
        Some(user) => (user.sn, user.nick_name, user.avatar_url),
        None => {
            let mut nick_name = match info.nickname.clone() {
                Some(n) => n,
                None => {
                    let rand_alpha = utils::rand::generate_alphanumeric_code(4);
                    format!("User_{rand_alpha}")
                }
            };
            let is_nick_error = info.validate().is_err();
            if is_nick_error {
                let rand_alpha = utils::rand::generate_alphanumeric_code(4);
                nick_name = format!("User_{rand_alpha}");
            }
            
            // 가입되어 있지 않은 상태, 가입처리
            for i in 0..=4 {
                let nick_is_some = super::user::nick_name_is_some(&mut tx, &nick_name).await?;
                if nick_is_some {
                    // 닉네임 변경 필요
                    let rand_alpha = utils::rand::generate_alphanumeric_code(4);
                    nick_name = format!("User_{rand_alpha}");
                } else {
                    break
                }
                if i == 4 {
                    Err(UserError::NickNameExists)?;
                }
            }

            let sequence = repositories::user::select_next_user_seq(&mut tx).await?;
            let user_sn = sequence.nextval as i32;

            let _insert = repositories::user::insert_user(
                &mut tx,
                info.profile_image.as_deref(),
                user_sn,
                &nick_name,
                &info.email,
                None,
                ProviderTyEnum::Naver,
                Some(&info.id),
                None,
                provider_access_token,
                provider_refresh_token,
                UserSttEnum::Ok,
            ).await?;
            (user_sn, nick_name, info.profile_image)
        }
    };

    // 로그인 처리

    let sequence = repositories::refresh_token::select_next_refresh_token_seq(&mut tx).await?;
    let chk = sequence.nextval;

    // 토큰클레임 생성
    let now: OffsetDateTime = OffsetDateTime::now_utc();
    let acc_exp = *utils::config::get_config_jwt_access_time();
    let refr_exp = *utils::config::get_config_jwt_refresh_time();
    let access_claims = AccessClaims::new(user_sn.to_string(), now + Duration::seconds(acc_exp), now, None, nick_name, avatar_url);
    let refresh_claims = RefreshClaims::new(user_sn.to_string(), now + Duration::seconds(refr_exp), now, None, chk as usize);

    // 토큰 생성
    let acc = utils::config::get_config_jwt_access_keys();
    let access_token = utils::jwt::generate_jwt(&access_claims, &acc.encoding)?;
    let refr = utils::config::get_config_jwt_refresh_keys();
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
            chk as i32,
            user_sn,
            refresh_token_hash,
            refresh_token.clone(),
            db_refr_exp_timestap,
            None,
            addr,
            user_agent,
        ).await?;

    repositories::tx::commit(tx).await?;
    Ok((access_token, refresh_token))
}