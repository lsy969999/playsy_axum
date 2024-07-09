#[derive(Debug)]
pub enum MyAuthError {
    MissingCredentials, // 인증토큰 미존재
    WrongCredential, // 잘못된 인증토큰
    TokenCreation, // 토큰 생성 에러
}

impl std::fmt::Display for MyAuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingCredentials => write!(f, "MissingCredentials"),
            Self::WrongCredential => write!(f, "WrongCredential"),
            Self::TokenCreation => write!(f, "TokenCreation"),
        }
    }
}

impl std::error::Error for MyAuthError {}