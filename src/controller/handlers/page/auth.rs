use askama::Template;
use axum::{ response::IntoResponse, Form};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use time::Duration;
use crate::{configs::{consts::{ACCESS_TOKEN, REFRESH_TOKEN}, errors::app_error::{PageHandlerLayerError, ServiceLayerError}, extractors::database_connection::DatabaseConnection, into_responses::html_template::HtmlTemplate}, controller::handlers::dto::auth::LoginAuthReqDto, services, utils};
use crate::configs::filters;

use super::fragment::user_info::UserInfo;
#[derive(Template)]
#[template(path="pages/auth.html")]
struct AuthTemplate {
    user_info: Option<UserInfo>,
    auth_form: AuthFormFragment
}

#[derive(Template)]
#[template(path="fragments/auth_form.html")]
struct AuthFormFragment {
    email_value: Option<String>,
    pass_value: Option<String>,
    email_err_msg: Option<String>,
    pass_err_msg: Option<String>,
}

impl AuthFormFragment {
    pub fn new(email_value: Option<String>, pass_value: Option<String>, email_err_msg: Option<String>, pass_err_msg: Option<String>,) -> Self {
        Self { email_value, pass_value, email_err_msg, pass_err_msg }
    }
}

impl Default for AuthFormFragment {
    fn default() -> Self {
        Self { email_value: None, pass_value: None, email_err_msg: None, pass_err_msg: None }
    }
}

#[derive(Template)]
#[template(path="fragments/auth_fail.html")]
struct AuthFailTemplate;

pub async fn auth_page() -> impl IntoResponse {
    HtmlTemplate(
        AuthTemplate {
            auth_form: AuthFormFragment::default(),
            user_info: None,
        }
    )
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
    DatabaseConnection(conn): DatabaseConnection,
    jar: CookieJar, 
    Form(form): Form<LoginAuthReqDto>,
) -> Result<impl IntoResponse, PageHandlerLayerError> {
    // 이메일 로그인 서비스 호출
    Ok(
        match services::auth::auth_email_request(conn, &form.email, &form.password).await {
            // 성공
            Ok((access_token, refresh_token)) => {
                let acc_time = utils::settings::get_jwt_access_time();
                let acc_token_cookie = Cookie::build((ACCESS_TOKEN, access_token))
                    .path("/")
                    .http_only(true)
                    .max_age(Duration::seconds(*acc_time));
                let refr_time = utils::settings::get_jwt_refresh_time();
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
                let af = AuthFormFragment::new(None, None, None, Some(msg.to_string()));
                HtmlTemplate(
                    af
                ).into_response()
            }
        }
    )
}