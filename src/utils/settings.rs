use crate::configs::settings::{Access, JwtKeys, Refresh, SETTINGS};

pub fn get_settings_jwt_access_keys() -> &'static JwtKeys<Access> {
    &SETTINGS.get().unwrap().jwt_access_keys
}

pub fn get_settings_jwt_refresh_keys() -> &'static JwtKeys<Refresh> {
    &SETTINGS.get().unwrap().jwt_refresh_keys
}