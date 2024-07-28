use askama::Template;
use super::filters;
use crate::models::entities::user::User;
use crate::models::user_info::UserInfo;
use crate::models::entities::user::ProviderTyEnum;

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
#[template(path="pages/join_email.html")]
pub struct JoinEmailTemplate {
    pub user_info: Option<UserInfo>,
}

#[derive(Template)]
#[template(path="fragments/join_email_error.html")]
pub struct JoinEmailErrorFragment {
    pub msgs: Vec<String>
}


#[derive(Template)]
#[template(path="pages/join_social.html")]
pub struct JoinSocialTemplate {
    pub user_info: Option<UserInfo>,
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


#[derive(Template)]
#[template(path="pages/join_email_success.html")]
pub struct JoinEamilSuccessTemplate {
    pub user_info: Option<UserInfo>,
}