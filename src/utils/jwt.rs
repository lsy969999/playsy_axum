use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{de::DeserializeOwned, Serialize};

pub fn generate_jwt<T>(
    claims: &T,
    encoding_key: &EncodingKey
) -> Result<String, jsonwebtoken::errors::Error>
    where T: Serialize {
    jsonwebtoken::encode(&Header::default(), claims, encoding_key)
}

pub fn decode_jwt<T>(
    token: &str,
    decoding_key: &DecodingKey
) -> Result<T, jsonwebtoken::errors::Error>
    where T: DeserializeOwned {
    let token_data = jsonwebtoken::decode::<T>(token, decoding_key, &Validation::default())?;
    Ok(token_data.claims)
}