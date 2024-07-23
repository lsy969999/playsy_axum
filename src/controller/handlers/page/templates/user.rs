use askama::Template;
use crate::configs::models::user_info::UserInfo;
use crate::configs::askama_filters as filters;
use crate::repositories::entities::user::User;

#[derive(Template)]
#[template(path="pages/join.html")]
pub struct JoinTemplate {
    pub user_info: Option<UserInfo>,
    pub join_form: JoinFormFragment,
}

#[derive(Template)]
#[template(path="fragments/join_form.html")]
pub struct JoinFormFragment {
    pub nick_name_value: Option<String>,
    pub email_value: Option<String>,
    pub pass_value: Option<String>,
    pub nick_name_err_msg: Option<String>,
    pub email_err_msg: Option<String>,
    pub pass_err_msg: Option<String>,
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
#[template(path="pages/mypage.html")]
pub struct MyPageTemplate {
    pub user_info: Option<UserInfo>,
    pub user: User
}

#[derive(Template)]
#[template(path="fragments/join_success.html")]
pub struct JoinSuccessFragment;
