use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiHandlerLayerError {
    #[error("Service Error")]
    Service(#[from] ServiceLayerError)
}

#[derive(Error, Debug)]
pub enum PageHandlerLayerError {
    #[error("Service Error")]
    Service(#[from] ServiceLayerError)
}

#[derive(Error, Debug)]
pub enum ServiceLayerError {
    #[error("Repository Layer")]
    Repository(#[from] RepositoryLayerError),
    #[error("Jwt error")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("User Error")]
    CustomUser(#[from] UserError),
    #[error("Crypto Error")]
    CustomCrypto(#[from] CryptoError),
    #[error("Auth Error")]
    CustomAuth(#[from] AuthError),
}

#[derive(Error, Debug)]
pub enum RepositoryLayerError {
    #[error("Database Error")]
    Db(#[from] sqlx::error::Error),
}

#[derive(Error, Debug)]
pub enum UserError {
    #[error("UserExists")]
    UserExists,
    #[error("UserNotExists")]
    UserNotExists,
    #[error("NickNameExists")]
    NickNameExists,
    #[error("PasswordGenFail")]
    PasswordGenFail,
    #[error("UserPasswordNotExists")]
    UserPasswordNotExists,
    #[error("UserPasswordNotMatch")]
    UserPasswordNotMatch,
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("TokenCreation")]
    TokenCreation
}

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Argon2GenFail")]
    Argon2GenFail,
    #[error("Argon2VerfyFail")]
    Argon2VerfyFail,
}