use crate::models::entities::user::{UserSttEnum, UserTyEnum};

pub struct GenAccessTokenArgs {
    pub user_sn: String,
    pub nick_name: String,
    pub avatar_url: Option<String>,
    pub user_stt: UserSttEnum,
    pub user_ty: UserTyEnum,
}

pub struct GenRefreshTokenArgs {
    pub user_sn: String,
    pub chk: usize,
}