use std::{collections::HashMap, error::Error, fmt};
use config::Config;
use jsonwebtoken::{DecodingKey, EncodingKey};

use sqlx::PgPool;
pub mod myconfig;
pub mod handlers;
pub mod routes;
pub mod models;
pub mod services;
pub mod middlewares;
pub mod errors;
pub mod into_responses;
pub mod repositories;
pub mod extractors;

/// 공유상태
#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    // pub jwt_keys: JwtKeys,
    pub jwt_access_keys: JwtAccessKeys,
    pub jwt_refresh_keys: JwtRefreshKeys,
}

// #[derive(Clone)]
// pub struct JwtKeys {
//     encoding: EncodingKey,
//     decoding: DecodingKey,
// }

// impl fmt::Debug for JwtKeys {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("JwtKeys")
//             .field("encoding", &"EncodingKey(...)")
//             .field("decoding", &"DecodingKey(...)")
//             .finish()
//     }
// }

// impl JwtKeys {
//     pub fn new(secret: &str) -> Self {
//         let bsecret = secret.as_bytes();
//         Self {
//             encoding: EncodingKey::from_secret(bsecret),
//             decoding: DecodingKey::from_secret(bsecret),
//         }
//     }
// }

#[derive(Clone)]
pub struct JwtAccessKeys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl fmt::Debug for JwtAccessKeys {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl fmt::Debug for JwtRefreshKeys {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

//
// --------
//


/// 그레이스풀 셧다운
/// https://github.com/tokio-rs/axum/blob/main/examples/graceful-shutdown/src/main.rs
pub async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };
    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .expect("failed to install signal handler")
        .recv()
        .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c => {
            println!("shoutdown ctrl_c")
        },
        _ = terminate => {
            println!("shoutdown terminate")
        },
    }
}

//
// --------
//

//
// --------
//

#[derive(Debug)]
pub struct Settings {
    pub database_url: String,
    pub server_port: String,
    // pub jwt_secret: String,
    pub jwt_access_secret: String,
    pub jwt_refresh_secret: String,
}

impl Settings {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let settings = Config::builder()
            .add_source(config::File::with_name("Settings"))
            .build()?;
        let hm = settings.try_deserialize::<HashMap<String, String>>()?;
        let dburl = hm.get("database_url").unwrap();
        let server_port = hm.get("server_port").unwrap();
        // let jwt_secret = hm.get("jwt_secret").unwrap();
        let jwt_access_secret = hm.get("jwt_access_secret").unwrap();
        let jwt_refresh_secret = hm.get("jwt_refresh_secret").unwrap();
        Ok(Settings {
            database_url: dburl.to_string(),
            server_port: server_port.to_string(),
            // jwt_secret: jwt_secret.to_string(),
            jwt_access_secret: jwt_access_secret.to_string(),
            jwt_refresh_secret: jwt_refresh_secret.to_string(),
        })
    }
}