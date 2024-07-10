use askama::Template;
use axum::{response::IntoResponse, Form};
use hyper::StatusCode;
use tracing::error;
use validator::Validate;
use crate::{configs::{errors::app_error::{PageHandlerLayerError, ServiceLayerError, UserError}, extractors::database_connection::DatabaseConnection, into_responses::html_template::HtmlTemplate}, controller::handlers::dto::user::JoinReqDto, services};

#[derive(Template)]
#[template(path="pages/join.html")]
struct JoinTemplate<'a> {
    join_form: JoinFormFragment<'a>
}

#[derive(Template)]
#[template(path="fragments/join_form.html")]
struct JoinFormFragment<'a> {
    nick_name_validate_txt: Option<&'a str>
}

#[derive(Template)]
#[template(path="fragments/join_success.html")]
struct JoinSuccessFragment;

/// 가입 페이지
pub async fn join_page() -> impl IntoResponse {
    HtmlTemplate(
        JoinTemplate {
            join_form: JoinFormFragment {
                nick_name_validate_txt: None
            }
        }
    )
}

/// 가입 요청
pub async fn join_request(
    DatabaseConnection(conn): DatabaseConnection,
    Form(form): Form<JoinReqDto>,
) -> Result<impl IntoResponse, PageHandlerLayerError> {
    // 파라미터 검증
    if let Err(error) = form.validate() {
        for (field, error) in error.field_errors() {
            error!("validate error, field: {:?}, error: {:?}", field, error);
        }
        return Ok((StatusCode::BAD_REQUEST, format!("파라미터 부정확")).into_response());
    }

    // 사용자 가입 서비스 호출
    Ok(
        match services::user::user_join_service(conn, form.nick_name, form.email, form.password).await {
            // 성공
            Ok(()) => {
                HtmlTemplate(JoinSuccessFragment).into_response()
            }
            // 실패 닉네임 중복
            Err(ServiceLayerError::CustomUser(UserError::NickNameExists)) => {
                HtmlTemplate(
                    JoinFormFragment {
                        nick_name_validate_txt: Some("사용자 닉네임이 이미 존재합니다.")
                    }
                ).into_response()
            }
            Err(err) => Err(err)?
        }
    )
}
