use askama::Template;
use crate::configs::models::user_info::UserInfo;
use crate::configs::askama_filters as filters;

#[derive(Template)]
#[template(path="pages/auth.html")]
pub struct AuthTemplate {
    pub user_info: Option<UserInfo>,
    pub auth_form: AuthFormFragment
}

#[derive(Template)]
#[template(path="fragments/auth_form.html")]
pub struct AuthFormFragment {
    pub authenticity_token: String,
    pub email_value: Option<String>,
    pub pass_value: Option<String>,
    pub email_err_msg: Option<String>,
    pub pass_err_msg: Option<String>,
}

impl AuthFormFragment {
    pub fn new(authenticity_token: String, email_value: Option<String>, pass_value: Option<String>, email_err_msg: Option<String>, pass_err_msg: Option<String>,) -> Self {
        Self { authenticity_token, email_value, pass_value, email_err_msg, pass_err_msg }
    }
}