use std::sync::Arc;
use askama::Template;
use axum::{extract::State, response::IntoResponse, Form};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use hyper::StatusCode;
use serde::Deserialize;
use time::{Duration, OffsetDateTime};
use jsonwebtoken::{encode, Header};
use tracing::info;
use crate::{into_responses::html_template::HtmlTemplate, models::auth::Claims, AppState};

#[derive(Template)]
#[template(path="pages/auth.html")]
struct AuthTemplate;

pub async fn auth_page() -> impl IntoResponse {
    HtmlTemplate(
        AuthTemplate
    )
}
#[derive(Template)]
#[template(path="fragments/auth_fail.html")]
struct AuthFailTemplate;

const FAKE_EMAIL: &str = "lsy@lsy.com";
const FAKE_PASSWORD: &str = "password";

#[derive(Deserialize, Debug)]
pub struct Input {
    pub email: String,
    pub password: String
}

pub async fn auth_request(
    State(state): State<Arc<AppState>>,
    jar: CookieJar, 
    Form(input): Form<Input>,
) -> impl IntoResponse {
    info!("input: {:?}", input);
    if input.email.is_empty() || input.password.is_empty() {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("Missing credentials")).into_response()
    }

    let db_email = FAKE_EMAIL;
    let db_password = FAKE_PASSWORD;
    if input.email != db_email || input.password != db_password {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("WorongCredentials")).into_response()
    }

    let now = OffsetDateTime::now_utc();

    // access token
    let a_exputc = (now + Duration::seconds(60)).unix_timestamp() as usize;
    let a_claims = Claims {
        sub: "".to_string(),
        exp: a_exputc,
    };
    let access_token = encode(&Header::default(), &a_claims, &state.jwt_access_keys.encoding);
    if let Err(_error) = access_token {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("TokenCreation")).into_response()
    }
    let access_token = access_token.unwrap();

    //refresh token
    let r_exputc = (now + Duration::seconds(60)).unix_timestamp() as usize;
    let r_claims = Claims {
        sub: "".to_string(),
        exp: r_exputc,
    };
    let refresh_token = encode(&Header::default(), &r_claims, &state.jwt_refresh_keys.encoding);
    if let Err(_error) = refresh_token {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("TokenCreation")).into_response()
    }
    let refresh_token = refresh_token.unwrap();

    if true {
        // 로그인 성공
        let acc_token_cookie = Cookie::build(("access_token", access_token))
            .http_only(true)
            .max_age(Duration::seconds(30));
        let jar: CookieJar = jar.add(acc_token_cookie);
        let ref_token_cookie = Cookie::build(("refresh_token", refresh_token))
            .http_only(true)
            .max_age(Duration::seconds(30));
        let jar: CookieJar = jar.add(ref_token_cookie);
        (jar, [("HX-Redirect", "/")]).into_response()
    } else {
        // 로그인 실패
        let jar = jar.remove("access_token");
        let jar = jar.remove("refresh_token");
        let html = AuthFailTemplate.render().unwrap();
        (jar, html).into_response()
    }
}