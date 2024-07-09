#[derive(Debug)]
pub enum AuthError {
    MissingCredentials, // 인증토큰 미존재
    WrongCredential, // 잘못된 인증토큰
    TokenCreation, // 토큰 생성 에러
    UserNotExists,
    UserPasswordNotExists,
    PasswordVerify,
    PasswordNotMatch,
    Db,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingCredentials => write!(f, "MissingCredentials"),
            Self::WrongCredential => write!(f, "WrongCredential"),
            Self::TokenCreation => write!(f, "TokenCreation"),
            Self::UserNotExists => write!(f, "user not exists"),
            Self::UserPasswordNotExists => write!(f, "user password not exists"),
            Self::PasswordVerify => write!(f, "password verify error"),
            Self::PasswordNotMatch => write!(f, "password not match"),
            Self::Db => write!(f, "db error")
        }
    }
}

impl std::error::Error for AuthError {}