use askama::Template;

use crate::models::user_info::UserInfo;
#[derive(Template)]
#[template(path="pages/inquery.html")]
pub struct InqueryTemplate {
    pub user_info: Option<UserInfo>
}

#[derive(Template)]
#[template(path="fragments/inquery_success.html")]
pub struct InquerySuccessFragment;