use askama::Template;
use super::filters;

#[derive(Template)]
#[template(path="pages/auth.html")]
pub struct AuthTemplate {
    pub authenticity_token: String,
}

#[derive(Template)]
#[template(path="fragments/auth_error.html")]
pub struct AuthErrorFragment {
    pub msg: String
}

#[derive(Template)]
#[template(path="pages/signup.html")]
pub struct SignupPage;

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