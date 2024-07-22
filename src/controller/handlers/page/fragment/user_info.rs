#[derive(Debug, Clone)]
pub struct UserInfo{
    pub user_sn: i32,
    pub nick_name: String,
    pub avatar_url: Option<String>,
}

impl std::fmt::Display for UserInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "nick_name: {}", self.nick_name)
    }
}