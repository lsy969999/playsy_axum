use std::{collections::HashMap, marker::PhantomData, sync::Arc};
use config::Config;
use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::OnceCell;

pub static SETTINGS: OnceCell<Arc<Settings>> = OnceCell::new();

pub async fn load_settings() -> Arc<Settings> {
    SETTINGS.get_or_init(|| {
        let settings: Settings = Settings::new();
        Arc::new(settings)
    }).clone()
}

#[derive(Debug)]
pub struct Settings {
    pub database_url: String,
    pub server_port: String,
    pub jwt_access_keys: JwtKeys<Access>,
    pub jwt_refresh_keys: JwtKeys<Refresh>,
}

impl Settings {
    pub fn new() -> Self {
        let settings = Config::builder()
            .add_source(config::File::with_name("Settings"))
            .build().unwrap();
        let hm = settings.try_deserialize::<HashMap<String, String>>().unwrap();
        let database_url = hm.get("database_url").unwrap().to_string();
        let server_port = hm.get("server_port").unwrap().to_string();
        let jwt_access_secret = hm.get("jwt_access_secret").unwrap();
        let jwt_refresh_secret = hm.get("jwt_refresh_secret").unwrap();
        let jwt_access_keys = JwtKeys::new(jwt_access_secret);
        let jwt_refresh_keys = JwtKeys::new(jwt_refresh_secret);
        Self {
            database_url,
            server_port,
            jwt_access_keys,
            jwt_refresh_keys
        }
    }
}

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
