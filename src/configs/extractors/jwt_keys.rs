//AppState 에서 SETTINGS 직접 사용으로 전환

// use std::sync::Arc;
// use axum::{async_trait, extract::{FromRef, FromRequestParts}, http::request::Parts};
// use hyper::StatusCode;
// use crate::configs::models::app_state::{AppState, JwtAccessKeys, JwtRefreshKeys};

// pub struct JwtKeys<T>(pub T);

// #[async_trait]
// impl<S> FromRequestParts<S> for JwtKeys<JwtAccessKeys>
// where 
//     Arc<AppState>: FromRef<S>,
//     S: Send + Sync,
// {
//     type Rejection = (StatusCode, String);

//     async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//         let state = Arc::from_ref(state);
//         let acc = JwtAccessKeys::from_ref(&state.jwt_access_keys);
//         Ok(Self(acc))
//     }
// }

// #[async_trait]
// impl<S> FromRequestParts<S> for JwtKeys<JwtRefreshKeys>
// where 
//     Arc<AppState>: FromRef<S>,
//     S: Send + Sync,
// {
//     type Rejection = (StatusCode, String);

//     async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//         let aa = String::from("asdf");

//         let state = Arc::from_ref(state);
//         let refr = JwtRefreshKeys::from_ref(&state.jwt_refresh_keys);
//         Ok(Self(refr))
//     }
// }