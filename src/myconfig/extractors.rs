

// #[async_trait]
// impl<S> FromRequestParts<S> for Claims
// where
//     Arc<AppState>: FromRef<S>,
//     S: Send + Sync
// {
//     type Rejection = AuthError;
//     async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//         let state = Arc::from_ref(state);
//         let TypedHeader(Authorization(bearer)) = parts
//             .extract::<TypedHeader<Authorization<Bearer>>>()
//             .await
//             .map_err(|_| AuthError::InvalidToken)?;
//         // Decode user data
//         let token_data = decode::<Claims>(bearer.token(), &state.jwt_keys.decoding, &Validation::default())
//             .map_err(|_| AuthError::InvalidToken)?;
//         Ok(token_data.claims)
//     }
// }

