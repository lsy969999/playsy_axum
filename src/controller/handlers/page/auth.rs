use askama::Template;
use axum::{ response::IntoResponse, Form};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use hyper::StatusCode;
use time::Duration;
use tracing::error;
use validator::Validate;
use crate::{configs::{consts::{ACCESS_TOKEN, REFRESH_TOKEN}, errors::app_error::PageHandlerLayerError, extractors::database_connection::DatabaseConnection, into_responses::html_template::HtmlTemplate}, controller::handlers::dto::auth::LoginAuthReqDto, services, utils};

#[derive(Template)]
#[template(path="pages/auth.html")]
struct AuthTemplate {
    form: AuthFormFragment
}

#[derive(Template)]
#[template(path="fragments/auth_form.html")]
struct AuthFormFragment;

#[derive(Template)]
#[template(path="fragments/auth_fail.html")]
struct AuthFailTemplate;

pub async fn auth_page() -> impl IntoResponse {
    HtmlTemplate(
        AuthTemplate {
            form: AuthFormFragment
        }
    )
}

pub async fn auth_email_request(
    DatabaseConnection(conn): DatabaseConnection,
    jar: CookieJar, 
    Form(form): Form<LoginAuthReqDto>,
) -> Result<impl IntoResponse, PageHandlerLayerError> {
    // 파라미터 검증
    if let Err(error) = form.validate() {
        for (field, error) in error.field_errors() {
            error!("validate error, field: {:?}, error: {:?}", field, error);
        }
        return Ok((StatusCode::BAD_REQUEST, format!("파라미터 부정확")).into_response());
    }

    // 이메일 로그인 서비스 호출
    Ok(
        match services::auth::auth_email_request(conn, form.email, form.password).await {
            // 성공
            Ok((access_token, refresh_token)) => {
                let acc_token_cookie = Cookie::build((ACCESS_TOKEN, access_token))
                    .http_only(true)
                    .max_age(Duration::seconds(1 * 60));
                let ref_token_cookie = Cookie::build((REFRESH_TOKEN, refresh_token))
                    .http_only(true)
                    .max_age(Duration::seconds(1 * 60 * 60));
                (jar.add(ref_token_cookie).add(acc_token_cookie), [("HX-Redirectxxxxx", "/")]).into_response()
            }
            // 실패
            Err(err) => Err(err)?
        }
    )
}