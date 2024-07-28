use validator::{ValidationError, ValidationErrors};

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

pub fn get_err_msg_vec(vec: Vec<ValidationError>) -> Vec<String> {
    let mut msgs = vec.iter().map(|ve|{
        match &ve.message {
            Some(m) => m.clone().into_owned(),
            None => String::new()
        }
    })
    .filter(|f| !f.is_empty())
    .collect::<Vec<String>>();
    msgs.sort();
    msgs
}