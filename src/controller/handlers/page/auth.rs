use askama::Template;
use axum::{ response::IntoResponse, Form};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use hyper::StatusCode;
use time::Duration;
use tracing::error;
use validator::Validate;
use crate::{configs::{consts::{ACCESS_TOKEN, REFRESH_TOKEN}, extractors::{database_connection::DatabaseConnection, jwt_keys::JwtKeys}, into_responses::html_template::HtmlTemplate, models::{app_state::{AppState, JwtAccessKeys, JwtRefreshKeys}, auth::Claims}}, controller::handlers::dto::auth::LoginAuthReqDto, services};

#[derive(Template)]
#[template(path="pages/auth.html")]
struct AuthTemplate;

#[derive(Template)]
#[template(path="fragments/auth_fail.html")]
struct AuthFailTemplate;

pub async fn auth_page() -> impl IntoResponse {
    HtmlTemplate(
        AuthTemplate
    )
}

pub async fn auth_request(
    JwtKeys(acc): JwtKeys<JwtAccessKeys>,
    JwtKeys(refr): JwtKeys<JwtRefreshKeys>,
    DatabaseConnection(conn): DatabaseConnection,
    jar: CookieJar, 
    Form(form): Form<LoginAuthReqDto>,
) -> impl IntoResponse {
    // 파라미터 검증
    if let Err(error) = form.validate() {
        for (field, error) in error.field_errors() {
            error!("validate error, field: {:?}, error: {:?}", field, error);
        }
        return (StatusCode::BAD_REQUEST, format!("파라미터 부정확")).into_response();
    }

    // 이메일 로그인 서비스 호출
    match services::auth::auth_email_request(conn, form.email, form.password, &acc.encoding, &refr.encoding).await {
        // 성공
        Ok((access_token, refresh_token)) => {
            let acc_token_cookie = Cookie::build((ACCESS_TOKEN, access_token))
                .http_only(true)
                .max_age(Duration::seconds(1 * 60));
            let jar: CookieJar = jar.add(acc_token_cookie);
            let ref_token_cookie = Cookie::build((REFRESH_TOKEN, refresh_token))
                .http_only(true)
                .max_age(Duration::seconds(1 * 60 * 60));
            let jar: CookieJar = jar.add(ref_token_cookie);
            (jar, [("HX-Redirect", "/")]).into_response()
        }
        // 실패
        Err(_) => {
            let jar = jar.remove(ACCESS_TOKEN);
            let jar = jar.remove(REFRESH_TOKEN);
            let html = AuthFailTemplate.render().unwrap();
            (jar, html).into_response()
        },
    }
}