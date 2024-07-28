use chrono::{DateTime, Utc};

pub fn nf(s: &Option<String>) -> ::askama::Result<String> {
    if let Some(s) = s {
        Ok(String::from(s))
    } else {
        Ok(String::from(""))
    }
}

pub fn format_datetime(value: &DateTime<Utc>) -> ::askama::Result<String> {
    Ok(value.format("%Y-%m-%d %H:%M:%S").to_string())
}

pub fn to_timestamp(value: &DateTime<Utc>) -> ::askama::Result<i64> {
    Ok(value.timestamp_millis())
}