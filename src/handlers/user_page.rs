use std::sync::Arc;

use askama::Template;
use axum::{extract::{Query, State}, response::{Html, IntoResponse}, Form};
use hyper::StatusCode;
use serde::Deserialize;
use sqlx::types::chrono::Utc;
use tracing::{error, info};
use validator::Validate;

use crate::{models::db::{code::DB_CODE, user::User}, AppState};

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
    State(state): State<Arc<AppState>>,
    Form(form): Form<JoinForm>
) -> impl IntoResponse {
    if let Err(error) = form.validate() {
        
        for (field, error) in error.field_errors() {
            error!("validate error, field: {:?}, error: {:?}", field, error);
        }
        
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("")).into_response();
    }

    let mut tx = match (&state.db_pool).begin().await {
        Ok(tx) => tx,
        Err(error) => {
            error!("error: {:?}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("")).into_response();
        }
    };

    // 닉네임 중복 체크
    let user = sqlx::query_as!(
        User,
        r#"
            SELECT *
            FROM tb_user
            WHERE nick_name = $1
        "#, 
        form.nick_name
    )
    .fetch_optional(&mut *tx)
    .await;

    // 쿼리 오류
    if let Err(error) = user {
        error!("error: {:?}", error);
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("")).into_response();
    }

    // 이미 존재
    if let Some(_data) = user.unwrap() {
        let template = JoinFormFragment {
            nick_name_validate_txt: Some("이미 존재합니다.")
        };
        match template.render() {
            Ok(html) => {
                return Html(html).into_response();
            }
            Err(error) => {
                // 렌더 에러
                error!("error: {:?}", error);
                return (StatusCode::INTERNAL_SERVER_ERROR, format!("")).into_response();
            }
        }
    }

    let now = Utc::now();
    let join = sqlx::query!(
        r#"
            INSERT INTO tb_user
            (
                nick_name, login_ty_cd, email, password, user_stt_cd, 
                user_ty_cd, created_at, created_by, updated_at, updated_by
            )
            VALUES
            (
                $1, $2, $3, $4, $5,
                $6, $7, $8, $9, $10
            )
        "#,
        form.nick_name,
        DB_CODE.login_ty_cd.email,
        form.email,
        form.password,
        DB_CODE.user_stt_cd.ok,
        DB_CODE.user_ty_cd.user,
        now,
        1,
        now,
        1
    )
    .execute(&mut *tx)
    .await;

    match join {
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
    let template = JoinSuccessFragment;
    match template.render() {
        Ok(html) => {
            return Html(html).into_response();
        }
        Err(error) => {
            // 렌더 에러
            error!("error: {:?}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("")).into_response();
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