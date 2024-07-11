use askama::Template;
use axum::{response::IntoResponse, Form};
use validator::Validate;
use crate::{configs::{errors::app_error::{PageHandlerLayerError, ServiceLayerError, UserError}, extractors::database_connection::DatabaseConnection, into_responses::html_template::HtmlTemplate}, controller::handlers::dto::user::JoinReqDto, services, utils};
use crate::configs::filters;

#[derive(Template)]
#[template(path="pages/join.html")]
struct JoinTemplate {
    join_form: JoinFormFragment
}

#[derive(Template)]
#[template(path="fragments/join_form.html")]
struct JoinFormFragment {
    nick_name_value: Option<String>,
    email_value: Option<String>,
    pass_value: Option<String>,
    nick_name_err_msg: Option<String>,
    email_err_msg: Option<String>,
    pass_err_msg: Option<String>,
}
impl JoinFormFragment {
    pub fn new(
        nick_name_value: Option<String>,
        email_value: Option<String>,
        pass_value: Option<String>,
        nick_name_err_msg: Option<String>,
        email_err_msg: Option<String>,
        pass_err_msg: Option<String>,
    ) -> Self {
        Self { nick_name_value, email_value, pass_value, nick_name_err_msg, email_err_msg, pass_err_msg }
    }
}
impl Default for JoinFormFragment {
    fn default() -> Self {
        Self { nick_name_value: None, email_value: None, pass_value: None, nick_name_err_msg: None, email_err_msg: None, pass_err_msg: None }
    }
}

#[derive(Template)]
#[template(path="fragments/join_success.html")]
struct JoinSuccessFragment;

/// 가입 페이지
pub async fn join_page() -> impl IntoResponse {
    HtmlTemplate(
        JoinTemplate {
            join_form: JoinFormFragment::default()
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
        match services::user::user_join_service(conn, form.nick_name, form.email, form.password).await {
            // 성공
            Ok(()) => {
                HtmlTemplate(JoinSuccessFragment).into_response()
            }
            // 실패 닉네임 중복
            Err(ServiceLayerError::CustomUser(UserError::NickNameExists)) => {
                HtmlTemplate(
                    JoinFormFragment::default()
                ).into_response()
            }
            Err(err) => Err(err)?
        }
    )
}
