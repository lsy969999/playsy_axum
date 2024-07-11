use std::borrow::Cow;
use regex::Regex;
use validator::ValidationError;

pub fn pass_vali_len_8(password: &str) -> Result<(), ValidationError>{
    // Password must be at least 8 characters long
    let code = "pv1";
    let m = "비밀번호는 8자 이상이어야 합니다.";
    if password.len() < 8 {
        return Err(
            ValidationError::new(code).with_message(Cow::Borrowed(m))
            );
    }
    Ok(())
}

pub fn pass_vali_1_upper(password: &str) -> Result<(), ValidationError>{
    // Password must contain at least one uppercase letter
    let code = "pv2";
    let m = "비밀번호에는 대문자가 하나 이상 포함되어야 합니다.";
    let has_uppercase = Regex::new(r"[A-Z]").unwrap();
    if !has_uppercase.is_match(password) {
        return Err(
            ValidationError::new(code).with_message(Cow::Borrowed(m))
        );
    }
    Ok(())
}

pub fn pass_vali_1_lower(password: &str) -> Result<(), ValidationError>{
    // Password must contain at least one lowercase letter
    let code = "pv3";
    let m = "비밀번호에는 소문자가 하나 이상 포함되어야 합니다.";
    let has_lowercase = Regex::new(r"[a-z]").unwrap();
    if !has_lowercase.is_match(password) {
        return Err(
            ValidationError::new(code).with_message(Cow::Borrowed(m))
        );
    }
    Ok(())
}

pub fn pass_vali_1_num(password: &str) -> Result<(), ValidationError>{
    //Password must contain at least one digit
    let code = "pv4";
    let m = "비밀번호에는 숫자가 1개 이상 포함되어야 합니다.";
    let has_digit = Regex::new(r"\d").unwrap();
    if !has_digit.is_match(password) {
        return Err(
            ValidationError::new(code).with_message(Cow::Borrowed(m))
        );
    }
    Ok(())
}

pub fn pass_vali_special_char(password: &str) -> Result<(), ValidationError>{
    // Password must contain at least one special character
    let code = "pv5";
    let m = "비밀번호에는 특수 문자가 하나 이상 포함되어야 합니다.";
    let has_special_char = Regex::new(r#"[!@#$%^&*(),.?\":{}|<>]"#).unwrap();
    if !has_special_char.is_match(password) {
        return Err(
            ValidationError::new(code).with_message(Cow::Borrowed(m))
        );
    }
    Ok(())
}