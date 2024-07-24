use axum_csrf::CsrfError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiHandlerLayerError {
    #[error("Service Error")]
    Service(#[from] ServiceLayerError)
}

#[derive(Error, Debug)]
pub enum PageHandlerLayerError {
    #[error("Service Error")]
    Service(#[from] ServiceLayerError),
    #[error("Csrf Error")]
    Csrf(#[from] CsrfError),
    #[error("Any")]
    Any(#[from] anyhow::Error),
    #[error("Auth Error")]
    Auth,
    #[error("Template Error")]
    Template(#[from] askama::Error)
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
    #[error("ParseJson Error")]
    ParseJson(#[from] serde_json::Error)
}

#[derive(Error, Debug)]
pub enum RepositoryLayerError {
    #[error("Database Error")]
    Db(#[from] sqlx::error::Error),
}

#[derive(Error, Debug)]
pub enum UserError {
    #[error("UserError")]
    UserError,
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