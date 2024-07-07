use std::sync::Arc;
use axum::{extract::State, Json};
use jsonwebtoken::{encode, Header};
use time::{Duration, OffsetDateTime};
use crate::{models::auth::{AuthBody, AuthPayload, Claims}, myconfig::error::AuthError, AppState};

const FAKE_EMAIL: &str = "lsy@lsy.com";
const FAKE_PASSWORD: &str = "password";

/*
 * requestToken
 * id, pass 받아서 비교
 * email,
 * password
 */
pub async fn request_token(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<AuthBody>, AuthError> {
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    let db_email = FAKE_EMAIL;
    let db_password = FAKE_PASSWORD;
    if payload.email != db_email || payload.password != db_password {
        return Err(AuthError::WorongCredentials)
    }

    let now = OffsetDateTime::now_utc();
    let exputc = (now + Duration::seconds(60)).unix_timestamp() as usize;

    let claims = Claims {
        sub: "".to_string(),
        exp: exputc,
    };

    let token = encode(&Header::default(), &claims, &state.jwt_keys.encoding)
    .map_err(|_|AuthError::TokenCreation)?;

    Ok(Json(AuthBody::new(token)))
}

pub async fn protected_url(claims: Claims) -> Result<String, AuthError> {
    Ok(format!("welcome! claims is {:?}", claims))
}