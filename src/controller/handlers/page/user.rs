
use askama::Template;
use axum::{extract::Query, response::{Html, IntoResponse}, Form};
use hyper::StatusCode;
use serde::Deserialize;
use tracing::{error, info};
use validator::Validate;

use crate::{configs::{errors::user_join::UserJoinError, extractors::database_connection::DatabaseConnection, into_responses::html_template::HtmlTemplate}, services};



#[derive(Template)]
#[template(path="pages/join.html")]
struct JoinTemplate<'a> {
    nick_name_validate_txt: Option<&'a str>
}



pub async fn join_page() -> impl IntoResponse {
    let template = JoinTemplate {
        nick_name_validate_txt: None
    };
    match template.render() {
        Ok(html) => {
            Html(html).into_response()
        }
        Err(error) => {
            error!("error: {}", error);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("page render error")).into_response()
        }
    }
}

#[derive(Template)]
#[template(path="fragments/join_form.html")]
struct JoinFormFragment<'a> {
    nick_name_validate_txt: Option<&'a str>
}


#[derive(Template)]
#[template(path="fragments/join_success.html")]
struct JoinSuccessFragment;

#[derive(Deserialize, Debug, Validate)]
pub struct JoinForm {
    #[validate(length(min = 1, message = "email len min 1"))]
    pub email: String,
    #[validate(length(min = 1, message = "pass len min 1"))]
    pub password: String,
    pub nick_name: String,
}

pub async fn join_request(
    DatabaseConnection(conn): DatabaseConnection,
    Form(form): Form<JoinForm>,
) -> impl IntoResponse {
    // 폼 검증
    if let Err(error) = form.validate() {
        for (field, error) in error.field_errors() {
            error!("validate error, field: {:?}, error: {:?}", field, error);
        }
        return (StatusCode::BAD_REQUEST, format!("파라미터 부정확")).into_response();
    }

    // 사용자 가입
    match services::user::user_join_service(conn, form.nick_name, form.email, form.password).await {
        // 성공
        Ok(_) => {
            HtmlTemplate(JoinSuccessFragment).into_response()
        }
        // 실패 닉네임 중복
        Err(UserJoinError::NickNameExists) => {
            HtmlTemplate(
                JoinFormFragment {
                    nick_name_validate_txt: Some("사용자 닉네임이 이미 존재합니다.")
                }
            ).into_response()
        }
        // 실패 디비 에러
        Err(UserJoinError::Db)
            | Err(UserJoinError::InsertFail) => {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("서버에 문제가 생겼습니다.")).into_response()
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ValidateNickname{
    pub nick_name: String
}
pub async fn validate_nickname(
    Query(form): Query<ValidateNickname>
) -> impl IntoResponse {
    info!("query: {:?}", form);
    Html("OK").into_response()
}