use askama::Template;
use super::filters;
use crate::models::entities::user::User;
use crate::models::user_info::UserInfo;
use crate::models::entities::user::ProviderTyEnum;


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
#[template(path="pages/join_email_success.html")]
pub struct JoinEamilSuccessTemplate {
    pub user_info: Option<UserInfo>,
}

#[derive(Template)]
#[template(path="pages/join_social.html")]
pub struct JoinSocialTemplate {
    pub user_info: Option<UserInfo>,
}

//

#[derive(Template)]
#[template(path="pages/mypage.html")]
pub struct MyPageTemplate {
    pub user_info: Option<UserInfo>,
    pub user: User
}