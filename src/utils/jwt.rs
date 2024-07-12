use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use crate::configs::models::claims::Claims;

pub fn generate_jwt(
    claims: &Claims,
    encoding_key: &EncodingKey
) -> Result<String, jsonwebtoken::errors::Error> {
    jsonwebtoken::encode(&Header::default(), claims, encoding_key)
}

pub fn decode_jwt(
    token: &str,
    decoding_key: &DecodingKey
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = jsonwebtoken::decode::<Claims>(token, decoding_key, &Validation::default())?;
    Ok(token_data.claims)
}