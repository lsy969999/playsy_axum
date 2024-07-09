use jsonwebtoken::{EncodingKey, Header};
use crate::configs::models::auth::Claims;

pub fn generate_jwt(
    claims: &Claims,
    encoding_key: &EncodingKey
) -> Result<String, jsonwebtoken::errors::Error> {
    jsonwebtoken::encode(&Header::default(), claims, encoding_key)
}