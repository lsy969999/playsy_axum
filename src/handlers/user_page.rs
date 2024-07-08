use std::sync::Arc;

use askama::Template;
use axum::{extract::{Query, State}, response::{Html, IntoResponse}, Form};
use hyper::StatusCode;
use serde::Deserialize;
use sqlx::{types::chrono::Utc, Acquire};
use tracing::{error, info};
use validator::Validate;

use crate::{extractors::{database_connection::DatabaseConnection, repository::Repository}, into_responses::html_template::HtmlTemplate, repositories::user::UserRepo};



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
    DatabaseConnection(mut conn): DatabaseConnection,
    Repository(user_repo): Repository<UserRepo>,
    Form(form): Form<JoinForm>,
) -> impl IntoResponse {
    if let Err(error) = form.validate() {
        for (field, error) in error.field_errors() {
            error!("validate error, field: {:?}, error: {:?}", field, error);
        }
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("")).into_response();
    }

    let mut tx = match conn.begin().await {
        Ok(tx) => tx,
        Err(error) => {
            error!("error: {:?}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("")).into_response();
        }
    };

    
    match &user_repo.select_user_by_nick_name(&mut *tx, form.nick_name.clone()).await {
        Ok(None) => { }
        Ok(Some(_user)) => {
            return HtmlTemplate(
                JoinFormFragment {
                    nick_name_validate_txt: Some("이미 존재합니다.")
                }
            ).into_response();
        }
        Err(error) => {
            error!("error: {:?}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("")).into_response();
        }
    };

    match &user_repo.insert_user(&mut *tx, form.nick_name, form.email, form.password).await {
        Ok(result) => {
            info!("insert rows: {:?}", result.rows_affected());
        }
        Err(error) => {
            error!("error: {:?}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("")).into_response();
        }
    }

    if let Err(error) = tx.commit().await {
        error!("error: {:?}", error);
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("")).into_response();
    }

    // 가입 성공
    HtmlTemplate(JoinSuccessFragment).into_response()
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