#[derive(Debug, Clone)]
pub struct UserInfo{
    pub user_sn: i32,
    pub nick_name: String,
    pub avatar_url: Option<String>,
}

impl std::fmt::Display for UserInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "user_sn: {:?}, nick_name: {:?}, avatar_url: {:?}", self.user_sn, self.nick_name, self.avatar_url)
    }
}