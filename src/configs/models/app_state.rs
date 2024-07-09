use jsonwebtoken::{DecodingKey, EncodingKey};
use sqlx::PgPool;


/// 공유상태
#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    // pub jwt_keys: JwtKeys,
    pub jwt_access_keys: JwtAccessKeys,
    pub jwt_refresh_keys: JwtRefreshKeys,
}

impl AppState {
    pub fn new(db_pool: PgPool, jwt_access_key_str: &str, jwt_refresh_key_str: &str) -> Self {
        let jwt_access_keys = JwtAccessKeys::new(&jwt_access_key_str);
        let jwt_refresh_keys = JwtRefreshKeys::new(&jwt_refresh_key_str);
        Self {
            db_pool,
            jwt_access_keys,
            jwt_refresh_keys
        }
    }
}

#[derive(Clone)]
pub struct JwtAccessKeys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl std::fmt::Debug for JwtAccessKeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JwtAccessKeys")
            .field("encoding", &"EncodingKey(...)")
            .field("decoding", &"DecodingKey(...)")
            .finish()
    }
}

impl JwtAccessKeys {
    pub fn new(secret: &str) -> Self {
        let bsecret = secret.as_bytes();
        Self {
            encoding: EncodingKey::from_secret(bsecret),
            decoding: DecodingKey::from_secret(bsecret),
        }
    }
}

#[derive(Clone)]
pub struct JwtRefreshKeys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl std::fmt::Debug for JwtRefreshKeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JwtRefreshKeys")
            .field("encoding", &"EncodingKey(...)")
            .field("decoding", &"DecodingKey(...)")
            .finish()
    }
}

impl JwtRefreshKeys {
    pub fn new(secret: &str) -> Self {
        let bsecret = secret.as_bytes();
        Self {
            encoding: EncodingKey::from_secret(bsecret),
            decoding: DecodingKey::from_secret(bsecret),
        }
    }
}