use std::{collections::HashMap, error::Error, sync::Arc};
use config::Config;
use once_cell::sync::OnceCell;

pub static SETTINGS: OnceCell<Arc<Settings>> = OnceCell::new();

pub async fn load_settings() -> Arc<Settings> {
    SETTINGS.get_or_init(|| {
        let settings: Settings = Settings::new().unwrap(); //settings issue시 panic
        Arc::new(settings)
    }).clone()
}

#[derive(Debug)]
pub struct Settings {
    pub database_url: String,
    pub server_port: String,
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
        let jwt_access_secret = hm.get("jwt_access_secret").unwrap();
        let jwt_refresh_secret = hm.get("jwt_refresh_secret").unwrap();
        Ok(Settings {
            database_url: dburl.to_string(),
            server_port: server_port.to_string(),
            jwt_access_secret: jwt_access_secret.to_string(),
            jwt_refresh_secret: jwt_refresh_secret.to_string(),
        })
    }
}