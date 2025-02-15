use askama::Template;
use super::filters;
use crate::models::entities::user::User;
use crate::models::user_info::UserInfo;
use crate::models::entities::user::{ProviderTyEnum, UserSttEnum};


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

#[derive(Template)]
#[template(path="fragments/mypage_update_error.html")]
pub struct MyPageUpdateErrorFragment {
    pub msgs: Vec<String>
}

#[derive(Template)]
#[template(path="fragments/join_social_error.html")]
pub struct JoinSocailUpdateErrorFragment {
    pub msgs: Vec<String>
}

//

#[derive(Template)]
#[template(path="pages/email_verification.html")]
pub struct EmailVerificationTemplate {
    pub user_info: Option<UserInfo>,
}

#[derive(Template)]
#[template(path="fragments/email_verification_error.html")]
pub struct EmailVerificationErrorFragment {
    pub msgs: Vec<String>
}

#[derive(Template)]
#[template(path="pages/email_verification_success.html")]
pub struct EmailVerificationSuccessTemplate {
    pub user_info: Option<UserInfo>,
}