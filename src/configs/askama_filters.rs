pub fn nf(s: &Option<String>) -> ::askama::Result<String> {
    if let Some(s) = s {
        Ok(String::from(s))
    } else {
        Ok(String::from(""))
    }
}
