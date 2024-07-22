
use std::net::SocketAddr;

use askama::Template;
use axum::{ extract::{ConnectInfo, Query, State}, response::{IntoResponse, Redirect}, Form};
use axum_extra::{extract::{cookie::Cookie, CookieJar}, headers::UserAgent, TypedHeader};
use oauth2::{reqwest::async_http_client, AuthorizationCode, CsrfToken, Scope, TokenResponse};
use serde::Deserialize;
use time::Duration;
use crate::{configs::{consts::{ACCESS_TOKEN, REFRESH_TOKEN}, errors::app_error::{PageHandlerLayerError, ServiceLayerError, UserError}, etc::oauth2_naver_client::NaverClient, extractors::{database_connection::DatabaseConnection, ext_client_ip::ExtClientIp, redis_connection::RedisConnection}, into_responses::html_template::HtmlTemplate}, controller::handlers::dto::auth::LoginAuthReqDto, services, utils};
use crate::configs::askama_filters as filters;
use super::{fragment::user_info::UserInfo, user};

#[derive(Template)]
#[template(path="pages/auth.html")]
struct AuthTemplate {
    user_info: Option<UserInfo>,
    auth_form: AuthFormFragment
}

#[derive(Template)]
#[template(path="fragments/auth_form.html")]
struct AuthFormFragment {
    authenticity_token: String,
    email_value: Option<String>,
    pass_value: Option<String>,
    email_err_msg: Option<String>,
    pass_err_msg: Option<String>,
}

impl AuthFormFragment {
    pub fn new(authenticity_token: String, email_value: Option<String>, pass_value: Option<String>, email_err_msg: Option<String>, pass_err_msg: Option<String>,) -> Self {
        Self { authenticity_token, email_value, pass_value, email_err_msg, pass_err_msg }
    }
}

// impl Default for AuthFormFragment {
//     fn default() -> Self {
//         Self { email_value: None, pass_value: None, email_err_msg: None, pass_err_msg: None }
//     }
// }

#[derive(Template)]
#[template(path="fragments/auth_fail.html")]
struct AuthFailTemplate;

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

pub async fn logout(jar: CookieJar) -> impl IntoResponse {
    let acc_token_cookie = Cookie::build((ACCESS_TOKEN, ""))
        .path("/")
        .http_only(true)
        .max_age(Duration::seconds(0));
    let ref_token_cookie = Cookie::build((REFRESH_TOKEN, ""))
        .path("/")
        .http_only(true)
        .max_age(Duration::seconds(0));
    let jar = jar.remove(acc_token_cookie);
    let jar = jar.remove(ref_token_cookie);
    (jar, [("HX-Redirect", "/")])
}

pub async fn auth_email_request(
    token: axum_csrf::CsrfToken,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    TypedHeader(user_agnet): TypedHeader<UserAgent>,
    DatabaseConnection(conn): DatabaseConnection,
    jar: CookieJar, 
    Form(form): Form<LoginAuthReqDto>,
) -> Result<impl IntoResponse, PageHandlerLayerError> {
    //csrf
    token.verify(&form.authenticity_token)?;
    // 이메일 로그인 서비스 호출
    Ok(
        match services::auth::auth_email_request(conn, &form.email, &form.password, addr.to_string(), user_agnet.to_string()).await {
            // 성공
            Ok((access_token, refresh_token)) => {
                let acc_time = utils::config::get_config_jwt_access_time();
                let acc_token_cookie = Cookie::build((ACCESS_TOKEN, access_token))
                    .path("/")
                    .http_only(true)
                    .max_age(Duration::seconds(*acc_time));
                let refr_time = utils::config::get_config_jwt_refresh_time();
                let ref_token_cookie = Cookie::build((REFRESH_TOKEN, refresh_token))
                    .path("/")
                    .http_only(true)
                    .max_age(Duration::seconds(*refr_time));
                (jar.add(ref_token_cookie).add(acc_token_cookie), [("HX-Redirect", "/")]).into_response()
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
                let af = AuthFormFragment::new(token.authenticity_token().unwrap(), None, None, None, Some(msg.to_string()));
                (
                    token,
                    HtmlTemplate(
                        af
                    )
                ).into_response()
            }
        }
    )
}

pub async fn google_login() -> impl IntoResponse {
    let client = utils::oauth2::google_oauth2_client();
    let (authorize_url, _csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(utils::oauth2::google_oauth2_scope_email())
        .add_scope(utils::oauth2::google_oauth2_scope_profile())
        .url();
    Redirect::temporary(authorize_url.to_string().as_str())
}

#[derive(Debug, Deserialize)]
pub struct OAuthCallback {
    pub code: String,
    pub state: String,
}

pub async fn google_callback(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    DatabaseConnection(conn): DatabaseConnection,
    query: Query<OAuthCallback>,
    jar: CookieJar, 
) -> Result<impl IntoResponse, PageHandlerLayerError> {
    tracing::debug!("google_callback!! query: {:?}, addr: {:?}, user_agent: {:?}", query, addr, user_agent);
    let client = utils::oauth2::google_oauth2_client();
    let token_result = client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await;

    Ok(
        match token_result {
            Ok(token) => {
                tracing::debug!("token_result: {:?}", token);
                let at: &str = token.access_token().secret();
                let info = utils::oauth2::google_oauth2_user_info_api(at).await?;
                let tokens = services::auth::auth_google_request(
                    conn,
                    info,
                    Some(at),
                    addr.to_string(),
                    user_agent.to_string()
                ).await;

                match tokens {
                    Ok((access_token, refresh_token)) => {
                        let acc_time = utils::config::get_config_jwt_access_time();
                        let acc_token_cookie = Cookie::build((ACCESS_TOKEN, access_token))
                            .path("/")
                            .http_only(true)
                            .max_age(Duration::seconds(*acc_time));
                        let refr_time = utils::config::get_config_jwt_refresh_time();
                        let ref_token_cookie = Cookie::build((REFRESH_TOKEN, refresh_token))
                            .path("/")
                            .http_only(true)
                            .max_age(Duration::seconds(*refr_time));
                        (jar.add(ref_token_cookie).add(acc_token_cookie), Redirect::to("/")).into_response()
                    }
                    Err(err) => {
                        tracing::error!("google login service err {}", err);
                        Err(err)?
                    }
                }
            }
            Err(err) => {
                tracing::error!("google_callback error: {:?}", err);
                Err(anyhow::anyhow!(err))?
            }
        }
    )
}

pub async fn naver_login(
    State(client): State<NaverClient>
) -> impl IntoResponse {
    let client = utils::oauth2::naver_oauth2_client().await;
    let (authorize_url, _csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .url();
    tracing::debug!("aaaa1111");
    Redirect::temporary(authorize_url.to_string().as_str())
}

pub async fn naver_callback(
    State(client): State<NaverClient>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    DatabaseConnection(conn): DatabaseConnection,
    query: Query<OAuthCallback>,
    jar: CookieJar, 
) -> Result<impl IntoResponse, PageHandlerLayerError> {
    tracing::debug!("aaaa2222");
    tracing::debug!("naver_callback!! query: {:?}, addr: {:?}, user_agent: {:?}", query, addr, user_agent);
    let client = utils::oauth2::naver_oauth2_client().await;
    let token_result = client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .add_extra_param("grant_type", "authorization_code")
        .add_extra_param("state", query.state.clone())
        .request_async(async_http_client)
        .await;
    tracing::debug!("naver_callback!! token_result {:?}", token_result);
    Ok(match token_result {
        Ok(token) => {
            tracing::debug!("okokok {:?}", token);
            let at = token.access_token().secret();
            let rt = token.refresh_token().unwrap().secret();
            tracing::debug!("rtrtrt {:?}", rt);
            let info = utils::oauth2::naver_oauth2_user_info_api(at).await?;
            tracing::debug!("info: {:?}", info);
            let tokens = services::auth::auth_naver_request(
                conn,
                info,
                Some(at),
                Some(rt),
                addr.to_string(),
                user_agent.to_string()
            ).await;

            match tokens {
                Ok((access_token, refresh_token)) => {
                    let acc_time = utils::config::get_config_jwt_access_time();
                    let acc_token_cookie = Cookie::build((ACCESS_TOKEN, access_token))
                        .path("/")
                        .http_only(true)
                        .max_age(Duration::seconds(*acc_time));
                    let refr_time = utils::config::get_config_jwt_refresh_time();
                    let ref_token_cookie = Cookie::build((REFRESH_TOKEN, refresh_token))
                        .path("/")
                        .http_only(true)
                        .max_age(Duration::seconds(*refr_time));
                    (jar.add(ref_token_cookie).add(acc_token_cookie), Redirect::to("/")).into_response()
                }
                Err(err) => {
                    tracing::error!("google login service err {}", err);
                    Err(err)?
                }
            }
        }
        Err(err) => {
            tracing::error!("naver_callback error! {:?}", err);
            Err(anyhow::anyhow!(err))?
        }
    })
}