use axum::{extract::Query, response::{Html, IntoResponse, Redirect}, Form};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use time::Duration;
use validator::ValidateArgs;
use crate::{configs::{consts::{ACCESS_TOKEN, REFRESH_TOKEN}, errors::app_error::{PageHandlerLayerError, ServiceLayerError, UserError}, extractors::{database_connection::DatabaseConnection, ext_user_info::UserInfoForPage}, into_responses::html_template::HtmlTemplate, validator::JoinReqValiContext}, controller::handlers::dto::user::{JoinEmailReqDto, JoinNickNameReqDto, JoinReqDto}, services, utils};
use super::templates::user::{JoinFormFragment, JoinSuccessFragment, JoinTemplate, MyPageTemplate};

/// 가입 페이지
pub async fn join_page() -> impl IntoResponse {
    HtmlTemplate(
        JoinTemplate {
            user_info: None,
            join_form: JoinFormFragment::default()
        }
    )
}

pub async fn nick_validate(
    DatabaseConnection(mut conn): DatabaseConnection,
    Query(query): Query<JoinNickNameReqDto>,
) -> Result<impl IntoResponse, PageHandlerLayerError> {
    let nick_name_is_some = services::user::nick_name_is_some(&mut conn, &query.nick_name).await?;
    let val_ctx = JoinReqValiContext::new(nick_name_is_some, false); // email_is_some 강제로 넣어준다. 이건 Nick_chk니까
    if let Err(error) = query.validate_with_args(&val_ctx) {
        let nick_name_err_msg = utils::validator::get_validate_error_messages(&error, "nick_name", "<br/>")
            .unwrap_or("".to_string());
        return Ok(Html(nick_name_err_msg).into_response())
    }
    Ok(Html("").into_response())
}

pub async fn email_validate(
    DatabaseConnection(mut conn): DatabaseConnection,
    Query(query): Query<JoinEmailReqDto>,
) -> Result<impl IntoResponse, PageHandlerLayerError> {
    let email_is_some = services::user::user_and_ldtye_email_is_some(&mut conn, &query.email).await?;
    let val_ctx = JoinReqValiContext::new(false, email_is_some); // nick_is_some 강제로 넣어준다. 이건 Email_Chk니까
    if let Err(error) = query.validate_with_args(&val_ctx) {
        let nick_name_err_msg = utils::validator::get_validate_error_messages(&error, "email", "<br/>")
            .unwrap_or("".to_string());
        return Ok(Html(nick_name_err_msg).into_response())
    }
    Ok(Html("").into_response())
}

/// 가입 요청
pub async fn join_request(
    DatabaseConnection(mut conn): DatabaseConnection,
    Form(form): Form<JoinReqDto>,
) -> Result<impl IntoResponse, PageHandlerLayerError> {
    // 파라미터 검증
    // validator 가 async를 지원하지 않아서
    // 이곳에서 먼저 닉을 조회하고, valdator 로직태운다.
    let nick_name_is_some = services::user::nick_name_is_some(&mut conn, &form.nick_name).await?;
    let email_is_some = services::user::user_and_ldtye_email_is_some(&mut conn, &form.email).await?;
    let val_ctx = JoinReqValiContext::new(nick_name_is_some, email_is_some);
    if let Err(error) = form.validate_with_args(&val_ctx) {
        let nick_name_value = Some(form.nick_name);
        let email_value = Some(form.email);
        let pass_value = Some(form.password);
        let nick_name_err_msg = utils::validator::get_validate_error_messages(&error, "nick_name", "<br/>");
        let email_err_msg = utils::validator::get_validate_error_messages(&error, "email", "<br/>");
        let pass_err_msg = utils::validator::get_validate_error_messages(&error, "password", "<br/>");
        return Ok(HtmlTemplate(
                JoinFormFragment::new(nick_name_value, email_value, pass_value, nick_name_err_msg, email_err_msg, pass_err_msg)
            ).into_response())
    }

    // 사용자 가입 서비스 호출
    Ok(
        match services::user::user_join_service(conn, &form.nick_name, &form.email, &form.password).await {
            // 성공
            Ok(()) => {
                HtmlTemplate(JoinSuccessFragment).into_response()
            }
            // 실패 닉네임 중복
            // 실패 이미 존재하는 유저
            Err(ServiceLayerError::CustomUser(user_err)) => {
                let nick_name_value = Some(form.nick_name);
                let email_value = Some(form.email);
                let pass_value = Some(form.password);
                let mut nick_name_err_msg = None;
                let mut email_err_msg = None;
                match user_err {
                    UserError::UserExists => email_err_msg = Some("이미 존재하는 이메일 입니다.".to_string()),
                    UserError::NickNameExists => nick_name_err_msg = Some("이미 존재하는 닉네임 입니다.".to_string()),
                    err => Err(ServiceLayerError::CustomUser(err))?
                };
                HtmlTemplate(
                    JoinFormFragment::new(nick_name_value, email_value, pass_value, nick_name_err_msg, email_err_msg, None)
                ).into_response()
            }
            Err(err) => Err(err)?
        }
    )
}

pub async fn my_page(
    DatabaseConnection(conn): DatabaseConnection,
    UserInfoForPage(user_info): UserInfoForPage,
) -> Result<impl IntoResponse, PageHandlerLayerError> {
    let user = services::user::select_user(conn, user_info.user_sn).await?;
    Ok(
        HtmlTemplate(
            MyPageTemplate {
                user_info: Some(user_info),
                user,
            }
        )
    )
}

pub async fn user_withdrawl(
    DatabaseConnection(conn): DatabaseConnection,
    UserInfoForPage(user_info): UserInfoForPage,
    jar: CookieJar
) -> Result<impl IntoResponse, PageHandlerLayerError> {
    let _ = services::user::user_withdrawl(conn, user_info.user_sn).await?;
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
    Ok((jar, Redirect::to("/")))
}