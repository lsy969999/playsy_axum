use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct InqueryReqDto{
    pub subject: Option<String>,
    pub email: Option<String>,
    pub message: Option<String>,
}