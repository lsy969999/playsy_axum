use chrono::Utc;
use sqlx::{pool::PoolConnection, PgConnection, Postgres};
use tracing::error;
use validator::Validate;
use crate::{configs::errors::app_error::{CryptoError, ServiceLayerError, UserError}, models::{auth_result::AuthResult, entities::user::{ProviderTyEnum, User, UserSttEnum}, fn_args::{auth::{EmailLoginArgs, GoogleLoginArgs, NaverLoginArgs}, repo::{InsertRefreshTokenArgs, InsertUserArgs}, token::{GenAccessTokenArgs, GenRefreshTokenArgs}}}, repositories::{self, }, utils::{self}};

/// 이메일 로그인 요청 처리
pub async fn auth_email_request(
    mut conn: PoolConnection<Postgres>,
    args: EmailLoginArgs,
) -> Result<AuthResult, ServiceLayerError> {
    let mut tx = repositories::tx::begin(&mut conn).await?;

    // 이메일과, 로그인타입코드로 유저 조회
    let user_select = repositories::user::select_user_by_email_and_login_ty_cd(
            &mut tx,
            &args.email,
            ProviderTyEnum::Email
        ).await?;

    // 유저 존재 체크
    let user = match user_select {
        Some(user) => user,
        None => return Err(UserError::UserNotExists)?
    };

    // 패스워드 언랩
    let password_hash = match user.password.clone() {
        Some(pass) => pass,
        None => return Err(UserError::UserPasswordNotExists)?
    };

    // 해시 매치 검증
    let result = utils::hash::verify_argon2(&args.password, &password_hash)
        .map_err(|error| {
            error!("error {}", error);
            CryptoError::Argon2VerfyFail
        })?;

    // 비밀번호 틀리다면 실패
    if !result {
        return Err(UserError::UserPasswordNotMatch)?;
    }

    // 로그인 처리
    let auth_result = login_process(&mut tx, user, &args.addr, &args.user_agent).await?;

    repositories::tx::commit(tx).await?;
    
    Ok(auth_result)
}

/// 구글 소셜 로그인 처리
pub async fn auth_google_request<'a>(
    mut conn: PoolConnection<Postgres>,
    args: GoogleLoginArgs<'a>,
) -> Result<AuthResult, ServiceLayerError> {
    if args.info.email.is_none() {
        Err(UserError::UserError)?
    }

    let mut tx = repositories::tx::begin(&mut conn).await?;

    let user_select = repositories::user::select_user_by_provider_type_enum_and_provider_id(
        &mut tx,
        ProviderTyEnum::Google,
        args.info.sub.clone().as_str(),
        
    ).await?;
    let jsonb = serde_json::to_value(&args.info)?;
    // 미가입 상태는 가입시켜준다.
    let user = match user_select {
        Some(user) => {
            repositories::user::update_user_provider_by_sn(&mut tx, args.provider_access_token, None, jsonb, user.sn).await?;
            user
        },
        None => {
            let mut nick_name = args.info.name.clone().unwrap();
            let is_nick_error = args.info.validate().is_err();
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
            
            let user = repositories::user::insert_user(
                &mut tx,
                InsertUserArgs {
                    avatar_url: args.info.picture.as_deref(),
                    email: args.info.email.as_deref(),
                    nick_name: &nick_name,
                    password: None,
                    user_sn,
                    provider_access_token: args.provider_access_token,
                    provider_refresh_token: None,
                    provider_etc: Some(jsonb),
                    provider_id: args.info.sub.as_str(),
                    provider_ty_enum: ProviderTyEnum::Google,
                    user_stt_enum: UserSttEnum::Ok,
                }
            ).await?;

            user
        }
    };

    let auth_result = login_process(&mut tx, user, &args.addr, &args.user_agent).await?;

    repositories::tx::commit(tx).await?;

    Ok(auth_result)
}

/// 네이버 소셜 로그인 처리
pub async fn auth_naver_request<'a>(
    mut conn: PoolConnection<Postgres>,
    args: NaverLoginArgs<'a>
) -> Result<AuthResult, ServiceLayerError> {
    // if info.email.is_none() {
    //     Err(UserError::UserError)?
    // }
    let mut tx = repositories::tx::begin(&mut conn).await?;

    let user_select = repositories::user::select_user_by_provider_type_enum_and_provider_id(
        &mut tx,
        ProviderTyEnum::Naver,
        args.info.id.clone().as_str(),
    ).await?;
    
    let jsonb = serde_json::to_value(&args.info)?;
    // 미가입 상태는 가입시켜준다.
    let user = match user_select {
        Some(user) => {
            repositories::user::update_user_provider_by_sn(&mut tx, args.provider_access_token, args.provider_refresh_token, jsonb, user.sn).await?;
            user
        },
        None => {
            let mut nick_name = match args.info.nickname.clone() {
                Some(n) => n,
                None => {
                    let rand_alpha = utils::rand::generate_alphanumeric_code(4);
                    format!("User_{rand_alpha}")
                }
            };
            let is_nick_error = args.info.validate().is_err();
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

            let user = repositories::user::insert_user(
                &mut tx,
                InsertUserArgs {
                    avatar_url: args.info.profile_image.as_deref(),
                    email: Some(&args.info.email),
                    nick_name: &nick_name,
                    user_sn: user_sn,
                    password: None,
                    provider_access_token: args.provider_access_token,
                    provider_refresh_token: args.provider_refresh_token,
                    provider_etc: Some(jsonb),
                    provider_id: &args.info.id,
                    provider_ty_enum: ProviderTyEnum::Naver,
                    user_stt_enum: UserSttEnum::Ok
                }
            ).await?;
            user
        }
    };

    let auth_result = login_process(&mut tx, user, &args.addr, &args.user_agent).await?;

    repositories::tx::commit(tx).await?;
    
    Ok(auth_result)
}

/// 깃허브 소셜 로그인 처리
pub async fn auth_github_request() -> Result<(String, String), ServiceLayerError> {
    todo!()
}


/// 로그인 처리시  
/// 각 로그인별 공통처리 사항  
/// 토큰 발급받고, 리프레시토큰 인서트  
async fn login_process(tx: &mut PgConnection, user: User, addr: &str, user_agent: &str) -> Result<AuthResult, ServiceLayerError> {
    let sequence = repositories::refresh_token::select_next_refresh_token_seq(tx).await?;
    let chk = sequence.nextval;

    let access_token = utils::jwt::generate_access_token(GenAccessTokenArgs {
        avatar_url: user.avatar_url,
        nick_name: user.nick_name,
        user_sn: user.sn.to_string(),
    })?;

    let refresh_token = utils::jwt::generate_refresh_token(GenRefreshTokenArgs {
        chk: chk as usize,
        user_sn: user.sn.to_string()
    })?;

    let refresh_token_hash = utils::hash::hash_sha_256(&refresh_token);
    let db_refr_exp_timestap = Utc::now() + chrono::Duration::seconds(*utils::config::get_config_jwt_refresh_time());

    repositories::refresh_token::insert_refresh_token(
            tx,
            InsertRefreshTokenArgs {
                sn: chk as i32,
                user_sn: user.sn,
                refresh_token: &refresh_token,
                expires_at: db_refr_exp_timestap,
                addr: &addr,
                user_agent: &user_agent,
                forwarded_id: None,
                hash: &refresh_token_hash
            }
        ).await?;

    Ok(AuthResult { access_token, refresh_token })
}