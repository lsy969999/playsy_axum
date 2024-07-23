use axum_extra::extract::cookie::Cookie;
use time::Duration;
use crate::configs::consts::{ACCESS_TOKEN, REFRESH_TOKEN};

pub fn generate_access_token_cookie<'a>(token_str: String) -> Cookie<'a> {
    let acc_time = super::config::get_config_jwt_access_time();
    Cookie::build((ACCESS_TOKEN, token_str))
        .path("/")
        .http_only(true)
        .max_age(Duration::seconds(*acc_time))
        .build()
}

pub fn generate_refresh_token_cookie<'a>(token_str: String) -> Cookie<'a> {
    let refr_time = super::config::get_config_jwt_refresh_time();
    Cookie::build((REFRESH_TOKEN, token_str))
        .path("/")
        .http_only(true)
        .max_age(Duration::seconds(*refr_time))
        .build()
}

pub fn generate_access_token_remove_cookie<'a>() -> Cookie<'a> {
    Cookie::build((ACCESS_TOKEN, ""))
        .path("/")
        .http_only(true)
        .max_age(Duration::seconds(0))
        .build()
}

pub fn generate_refresh_token_remove_cookie<'a>() -> Cookie<'a> {
    Cookie::build((REFRESH_TOKEN, ""))
        .path("/")
        .http_only(true)
        .max_age(Duration::seconds(0))
        .build()
}