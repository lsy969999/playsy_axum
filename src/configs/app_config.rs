use std::sync::Arc;
use config::Config;
use once_cell::sync::OnceCell;
use serde::Deserialize;

use crate::models::jwt_keys::{Access, JwtKeys, Refresh};

pub static APP_CONFIG: OnceCell<Arc<AppConfig>> = OnceCell::new();

pub async fn load_settings() -> Arc<AppConfig> {
    APP_CONFIG.get_or_init(|| {
        let settings: Settings = Settings::new();
        let jas = &settings.jwt.jwt_access_secret.clone();
        let jrs = &settings.jwt.jwt_refresh_secret.clone();
        let app_config = AppConfig::new(settings, jas, jrs);
        Arc::new(app_config)
    }).clone()
}

#[derive(Debug, Deserialize)]
pub struct AwsSettings {
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,
    pub aws_region: String,
    pub aws_s3_bucket: String,
    pub aws_s3_bucket_url: String,
}

#[derive(Debug, Deserialize)]
pub struct AppSettings {
    pub is_prd: bool,
    pub server_protocol: String,
    pub server_host: String,
    pub server_port: u32,
    pub csrf_key: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub database_url: String,
}

#[derive(Debug, Deserialize)]
pub struct RedisSettings {
    pub redis_url: String,
}

#[derive(Debug, Deserialize)]
pub struct JwtSettings {
    pub jwt_access_secret: String,
    pub jwt_refresh_secret: String,
    pub jwt_access_time: i64,
    pub jwt_refresh_time: i64,
}

#[derive(Debug, Deserialize)]
pub struct SmtpSettings {
    pub smtp_from: String,
    pub smtp_user_name: String,
    pub smtp_password: String,
}

#[derive(Debug, Deserialize)]
pub struct Oauth2Settings {
    pub oauth2_google_client_id: String,
    pub oauth2_google_client_secret: String,
    pub oauth2_naver_client_id: String,
    pub oauth2_naver_client_secret: String,
    pub oauth2_github_client_id: String,
    pub oauth2_github_client_secret: String,
    pub oauth2_kakao_client_id: String,
    pub oauth2_kakao_client_secret: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub app: AppSettings,
    pub database: DatabaseSettings,
    pub redis: RedisSettings,
    pub jwt: JwtSettings,
    pub smtp: SmtpSettings,
    pub oauth2: Oauth2Settings,
    pub aws: AwsSettings,
}

impl Settings {
    pub fn new() -> Self {
        let builder = Config::builder()
            .add_source(config::File::with_name("Settings"))
            .add_source(config::File::with_name("Settings.local").required(false))
            .build()
            .unwrap();

        builder.try_deserialize().unwrap()
    }
}

#[derive(Debug)]
pub struct AppConfig {
    pub settings: Settings,
    pub jwt_access_keys: JwtKeys<Access>,
    pub jwt_refresh_keys: JwtKeys<Refresh>,
}

impl AppConfig {
    pub fn new(settings: Settings, jwt_access_secret: &str, jwt_refresh_secret: &str) -> Self {
        let jwt_access_keys = JwtKeys::new(jwt_access_secret);
        let jwt_refresh_keys = JwtKeys::new(jwt_refresh_secret);
        Self { settings, jwt_access_keys, jwt_refresh_keys }
    }
}