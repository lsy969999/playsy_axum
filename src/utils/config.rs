use crate::{configs::app_config::{AwsSettings, Oauth2Settings, SmtpSettings, APP_CONFIG}, models::jwt_keys::{Access, JwtKeys, Refresh}};

pub fn get_config_jwt_access_keys() -> &'static JwtKeys<Access> {
    &APP_CONFIG.get().unwrap().jwt_access_keys
}

pub fn get_config_jwt_refresh_keys() -> &'static JwtKeys<Refresh> {
    &APP_CONFIG.get().unwrap().jwt_refresh_keys
}

pub fn get_config_smtp() -> &'static SmtpSettings {
    &APP_CONFIG.get().unwrap().settings.smtp
}

pub fn get_config_jwt_access_time() -> &'static i64 {
    &APP_CONFIG.get().unwrap().settings.jwt.jwt_access_time
}

pub fn get_config_jwt_refresh_time() -> &'static i64 {
    &APP_CONFIG.get().unwrap().settings.jwt.jwt_refresh_time
}

pub fn get_config_oauth2() -> &'static Oauth2Settings {
    &APP_CONFIG.get().unwrap().settings.oauth2
}

pub fn is_prd() -> &'static bool {
    &APP_CONFIG.get().unwrap().settings.app.is_prd
}

pub fn get_config_aws() -> &'static AwsSettings {
    &APP_CONFIG.get().unwrap().settings.aws
}