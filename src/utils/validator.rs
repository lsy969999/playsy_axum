use validator::ValidationErrors;

/// validator 에서 정의한 에러메시지 뽑기...
/// N개의 에러가 seperator에 의해서 join 되어 하나의 string으로 나간다.
pub fn get_validate_error_messages(errors: &ValidationErrors, key: &str, seperator:  &str) -> Option<String>{
    errors
        .field_errors()
        .get_key_value(key)
        .and_then(|(_a, v)| {
            Some(v
                .iter()
                .map(|v| {
                    match &v.message {
                        Some(message) => message.clone().into_owned(),
                        None => String::from(""),
                    }
                })
                .filter(|f| f != "")
                .collect::<Vec<_>>()
                .join(seperator)
            )
        })
}