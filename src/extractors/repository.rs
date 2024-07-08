use std::sync::Arc;

use axum::{async_trait, extract::{FromRef, FromRequestParts}, http::request::Parts};
use hyper::StatusCode;

use crate::{repositories::user::UserRepo, AppState};

pub struct Repository<T>(pub T);

#[async_trait]
impl<S> FromRequestParts<S> for Repository<UserRepo>
where 
    Arc<AppState>: FromRef<S>,
    S: Send + Sync,  {
    type Rejection = (StatusCode, &'static str);
    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = Arc::from_ref(state);
        let r: UserRepo = UserRepo::from_ref(&state.repositories.user_repo);
        Ok(Self(r))
    }
}