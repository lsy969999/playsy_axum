use std::sync::Arc;
use axum::{async_trait, extract::{rejection::FormRejection, FromRef, FromRequest, FromRequestParts, Request}, http::request::Parts, Form, RequestPartsExt};
use axum_extra::{headers::{authorization::Bearer, Authorization}, TypedHeader};
use hyper::StatusCode;
use jsonwebtoken::{decode, Validation};
use serde::de::DeserializeOwned;
use sqlx::PgPool;
use validator::Validate;
use crate::{models::auth::Claims, myconfig::error::AppError, AppState};
use super::error::AuthError;

#[derive(Debug, Clone, Default)]
pub struct ValidatedForm<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedForm(value))
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    Arc<AppState>: FromRef<S>,
    S: Send + Sync
{
    type Rejection = AuthError;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = Arc::from_ref(state);
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode user data
        let token_data = decode::<Claims>(bearer.token(), &state.jwt_keys.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;
        Ok(token_data.claims)
    }
}


pub struct DatabaseConnection(pub sqlx::pool::PoolConnection<sqlx::Postgres>);
#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    Arc<AppState>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = Arc::from_ref(state);
        let pool = PgPool::from_ref(&state.db_pool);
        let conn = pool.acquire()
            .await
            .map_err(|err|(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

        Ok(Self(conn))
    }
}