use std::marker::PhantomData;
use jsonwebtoken::{DecodingKey, EncodingKey};

pub struct Access;
pub struct Refresh;

#[derive(Clone)]
pub struct JwtKeys<T> {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
    _marker: PhantomData<T>,
}

impl<T> std::fmt::Debug for JwtKeys<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JwtKeys")
            .field("encoding", &"EncodingKey(...)")
            .field("decoding", &"DecodingKey(...)")
            .finish()
    }
}

impl<T> JwtKeys<T> {
    pub fn new(secret: &str) -> Self {
        let bsecret = secret.as_bytes();
        Self {
            encoding: EncodingKey::from_secret(bsecret),
            decoding: DecodingKey::from_secret(bsecret),
            _marker: PhantomData,
        }
    }
}
