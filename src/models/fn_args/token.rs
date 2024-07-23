pub struct GenAccessTokenArgs {
    pub user_sn: String,
    pub nick_name: String,
    pub avatar_url: Option<String>
}

pub struct GenRefreshTokenArgs {
    pub user_sn: String,
    pub chk: usize,
}