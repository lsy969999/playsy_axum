
use std::net::SocketAddr;
use anyhow::anyhow;
use axum::{ extract::{ConnectInfo, Query}, response::{IntoResponse, Redirect}, Form};
use axum_extra::{extract::CookieJar, headers::UserAgent, TypedHeader};
use oauth2::{reqwest::async_http_client, AuthorizationCode, CsrfToken, Scope, TokenResponse};
use crate::{configs::{consts::HX_REDIRECT, errors::app_error::{PageHandlerLayerError, ServiceLayerError}}, extractors::database_connection::DatabaseConnection, models::{auth_result::AuthResult, fn_args::auth::{EmailLoginArgs, SocialLoginArgs}, request::{auth::LoginAuthReqDto, oauth2::OAuthCallback}}, responses::html_template::HtmlTemplate, services, templates::auth::{AuthFormFragment, AuthTemplate}, utils};

/// 로그인 페이지
pub async fn auth_page(token: axum_csrf::CsrfToken) -> impl IntoResponse {
    let authenticity_token = token.authenticity_token().unwrap();
    (
        token, 
        HtmlTemplate(
            AuthTemplate {
                auth_form: AuthFormFragment {
                    authenticity_token,
                    email_value: None,
                    pass_value: None,
                    email_err_msg: None,
                    pass_err_msg: None,
                },
                user_info: None,
            }
        )
    ).into_response()
}

/// 로그아웃
pub async fn logout(jar: CookieJar) -> impl IntoResponse {
    let acc_token_cookie = utils::cookie::generate_access_token_remove_cookie();
    let ref_token_cookie = utils::cookie::generate_refresh_token_remove_cookie();
    (jar.remove(acc_token_cookie).remove(ref_token_cookie), Redirect::to("/?"))
}

/// 이메일 로그인
pub async fn email_login(
    jar: CookieJar, 
    csrf: axum_csrf::CsrfToken,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    TypedHeader(user_agnet): TypedHeader<UserAgent>,
    DatabaseConnection(conn): DatabaseConnection,
    Form(form): Form<LoginAuthReqDto>,
) -> Result<impl IntoResponse, PageHandlerLayerError> {
    //csrf
    csrf.verify(&form.authenticity_token)?;
    // 이메일 로그인 서비스 호출
    Ok(
        match services::auth::auth_email_request(
            conn,
            EmailLoginArgs{
                email: form.email,
                password: form.password,
                addr: addr.to_string(),
                user_agent: user_agnet.to_string()
            }).await {
            // 성공
            Ok(AuthResult {access_token, refresh_token}) => {
                let acc_token_cookie = utils::cookie::generate_access_token_cookie(access_token);
                let ref_token_cookie = utils::cookie::generate_refresh_token_cookie(refresh_token);
                (jar.add(ref_token_cookie).add(acc_token_cookie), [(HX_REDIRECT, "/")]).into_response()
            }
            // 실패
            Err(err) => {
                let msg = match err {
                    ServiceLayerError::CustomAuth(_) => {
                        "관리자에게 문의하세요."
                    }
                    ServiceLayerError::CustomCrypto(_) => {
                        "관리자에게 문의하세요."
                    }
                    ServiceLayerError::CustomUser(_) => {
                        "아이디 또는 비밀번호가 잘못되었습니다."
                    }
                    _ => Err(err)?
                };
                let authenticity_token = csrf.authenticity_token().unwrap();
                (
                    csrf,
                    HtmlTemplate(
                        AuthFormFragment {
                            authenticity_token,
                            email_err_msg: None,
                            email_value: None,
                            pass_value: None,
                            pass_err_msg: Some(msg.to_string())
                        }
                    )
                ).into_response()
            }
        }
    )
}

/// 구글 소셜 로그인
pub async fn google_login() -> impl IntoResponse {
    let client = utils::oauth2::google_oauth2_client();
    let (authorize_url, _csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(format!("https://www.googleapis.com/auth/userinfo.email")))
        .add_scope(Scope::new(format!("https://www.googleapis.com/auth/userinfo.profile")))
        .url();
    Redirect::temporary(authorize_url.to_string().as_str())
}

/// 구글 소셜 로그인 Oauth2 콜백
pub async fn google_login_callback(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    DatabaseConnection(conn): DatabaseConnection,
    query: Query<OAuthCallback>,
    jar: CookieJar, 
) -> Result<impl IntoResponse, PageHandlerLayerError> {
    let client = utils::oauth2::google_oauth2_client();
    let token_result = client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
        .map_err(|err| anyhow!(err))?;

    let at = token_result.access_token().secret().as_str();
    let rt = match token_result.refresh_token() {
        Some(r) => Some(r.secret().as_str()),
        None => None
    };
    let info = utils::oauth2::google_oauth2_user_info_api(at).await?;
    tracing::info!("google_login_callback!! query: {:?}, addr: {:?}, user_agent: {:?}, info: {:?}", query, addr, user_agent, info);

    let AuthResult {access_token, refresh_token} = services::auth::auth_google_request(
        conn,
        SocialLoginArgs {
            info,
            provider_access_token: Some(at),
            provider_refresh_token: rt,
            addr: addr.to_string(),
            user_agent: user_agent.to_string(),
        }
    ).await?;

    let acc_token_cookie = utils::cookie::generate_access_token_cookie(access_token);
    let ref_token_cookie = utils::cookie::generate_refresh_token_cookie(refresh_token);

    Ok( (jar.add(ref_token_cookie).add(acc_token_cookie), Redirect::to("/")).into_response() )
}

/// 네이버 소셜 로그인 
pub async fn naver_login(
) -> impl IntoResponse {
    let client = utils::oauth2::naver_oauth2_client();
    let (authorize_url, _csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .url();
    Redirect::temporary(authorize_url.to_string().as_str())
}

/// 네이버 소셜 로그인 Oauth2 콜백
pub async fn naver_login_callback(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    DatabaseConnection(conn): DatabaseConnection,
    query: Query<OAuthCallback>,
    jar: CookieJar, 
) -> Result<impl IntoResponse, PageHandlerLayerError> {
    let client = utils::oauth2::naver_oauth2_client();
    let token_result = client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .add_extra_param("grant_type", "authorization_code")
        .add_extra_param("state", query.state.clone())
        .request_async(async_http_client)
        .await
        .map_err(|err| anyhow!(err))?;
    
    let at = token_result.access_token().secret();
    let rt = match token_result.refresh_token() {
        Some(r) => Some(r.secret().as_str()),
        None => None
    };
    let info = utils::oauth2::naver_oauth2_user_info_api(at).await?;
    tracing::info!("naver_login_callback!! query: {:?}, addr: {:?}, user_agent: {:?}, info: {:?}", query, addr, user_agent, info);
    
    let AuthResult {access_token, refresh_token} = services::auth::auth_naver_request(
        conn,
        SocialLoginArgs {
            info,
            provider_access_token: Some(at),
            provider_refresh_token: rt,
            addr: addr.to_string(),
            user_agent: user_agent.to_string(),
        }
    ).await?;

    let acc_token_cookie = utils::cookie::generate_access_token_cookie(access_token);
    let ref_token_cookie = utils::cookie::generate_refresh_token_cookie(refresh_token);

    Ok( (jar.add(ref_token_cookie).add(acc_token_cookie), Redirect::to("/")).into_response() )
}

/// 깃헙 소셜 로그인 
pub async fn github_login() -> impl IntoResponse {
    let client = utils::oauth2::github_oauth2_client();
    let (authorize_url, _csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(format!("read:user")))
        .add_scope(Scope::new(format!("user:email")))
        .url();
    Redirect::temporary(authorize_url.to_string().as_str())
}

/// 깃헙 소셜 로그인 Oauth2 콜백
pub async fn github_login_callback(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    DatabaseConnection(conn): DatabaseConnection,
    query: Query<OAuthCallback>,
    jar: CookieJar, 
) -> Result<impl IntoResponse, PageHandlerLayerError> {
    let client = utils::oauth2::github_oauth2_client();
    let token_result = client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
        .map_err(|err| anyhow!(err))?;
    
    let at = token_result.access_token().secret().as_str();
    let rt = match token_result.refresh_token() {
        Some(r) => Some(r.secret().as_str()),
        None => None
    };
    let info = utils::oauth2::github_oauth2_user_info(at).await?;
    tracing::info!("github_callback!! query: {:?}, addr: {:?}, user_agent: {:?}, info: {:?}", query, addr, user_agent, info);

    let AuthResult {access_token, refresh_token} = services::auth::auth_github_request(
        conn,
        SocialLoginArgs {
            info,
            provider_access_token: Some(at),
            provider_refresh_token: rt,
            addr: addr.to_string(),
            user_agent: user_agent.to_string(),
        }
    ).await?;

    let acc_token_cookie = utils::cookie::generate_access_token_cookie(access_token);
    let ref_token_cookie = utils::cookie::generate_refresh_token_cookie(refresh_token);

    Ok( (jar.add(ref_token_cookie).add(acc_token_cookie), Redirect::to("/")).into_response() )
}

/// 카카오 소셜 로그인
pub async fn kakao_login() -> impl IntoResponse {
    let client = utils::oauth2::kakao_oauth2_client();
    let (authorize_url, _csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(format!("profile_nickname")))
        .add_scope(Scope::new(format!("profile_image")))
        // .add_extra_param("response_type", "code")
        .url();
    Redirect::temporary(authorize_url.to_string().as_str())
}

/// 카카오 소셜 로그인 Oauth2 콜백
pub async fn kakao_login_callback(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    DatabaseConnection(conn): DatabaseConnection,
    query: Query<OAuthCallback>,
    jar: CookieJar, 
) -> Result<impl IntoResponse, PageHandlerLayerError> {
    let client = utils::oauth2::kakao_oauth2_client();
    let token_result = client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .add_extra_param("state", query.state.clone())
        .add_extra_param("client_id", format!("{}", utils::config::get_config_oauth2().oauth2_kakao_client_id))
        .add_extra_param("client_secret", format!("{}", utils::config::get_config_oauth2().oauth2_kakao_client_secret))
        .request_async(async_http_client)
        .await
        .map_err(|err| anyhow!(err))?;
    let at = token_result.access_token().secret().as_str();
    let rt = match token_result.refresh_token() {
        Some(r) => Some(r.secret().as_str()),
        None => None
    };
    
    let info = utils::oauth2::kakao_oauth2_user_info(at).await?;
    tracing::info!("kakao_callback!! query: {:?}, addr: {:?}, user_agent: {:?}, info: {:?}", query, addr, user_agent, info);

    let AuthResult {access_token, refresh_token} = services::auth::auth_kakao_request(
        conn,
        SocialLoginArgs {
            info,
            provider_access_token: Some(at),
            provider_refresh_token: rt,
            addr: addr.to_string(),
            user_agent: user_agent.to_string(),
        }
    ).await?;

    let acc_token_cookie = utils::cookie::generate_access_token_cookie(access_token);
    let ref_token_cookie = utils::cookie::generate_refresh_token_cookie(refresh_token);

    Ok( (jar.add(ref_token_cookie).add(acc_token_cookie), Redirect::to("/")).into_response() )
}

/// 디스코드 소셜 로그인
pub async fn discord_login() ->impl IntoResponse {
    todo!()
}

/// 디스코드 소셜 로그인 Oauth2 콜백
pub async fn discord_login_callback() -> Result<(), PageHandlerLayerError> {
    todo!()
}

/// 애플 소셜 로그인
pub async fn apple_login() -> impl IntoResponse {
    todo!()
}

/// 애플 소셜 로그인 Oauth2 콜백
pub async fn apple_login_callback() -> Result<(), PageHandlerLayerError> {
    todo!()
}

