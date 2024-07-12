use crate::configs::settings::{Access, JwtKeys, Refresh, SmtpInfo, SETTINGS};

pub fn get_settings_jwt_access_keys() -> &'static JwtKeys<Access> {
    &SETTINGS.get().unwrap().jwt_access_keys
}

pub fn get_settings_jwt_refresh_keys() -> &'static JwtKeys<Refresh> {
    &SETTINGS.get().unwrap().jwt_refresh_keys
}

pub fn get_settings_smtp_info() -> &'static SmtpInfo {
    &SETTINGS.get().unwrap().smtp_info
}

pub fn get_jwt_access_time() -> &'static i64 {
    &SETTINGS.get().unwrap().jwt_access_time
}

pub fn get_jwt_refresh_time() -> &'static i64 {
    &SETTINGS.get().unwrap().jwt_refresh_time
}