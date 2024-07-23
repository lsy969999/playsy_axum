use crate::configs::app_config::Settings;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
}

#[test]
fn test_rand_alphanumeric_code() {
    let code = crate::utils::rand::generate_alphanumeric_code(10);
    println!("code: {}", code);
}

#[test]
fn settings_test() {
    let settings = Settings::new();
    println!("settings: {:?}", settings)
}

/// deadlock 발생 케이스  
/// 아래 상황의 임시적 해결방법은 블롭스코프를 명시적으로 작성하여  
/// 뮤텍스의 락 반환을 제어한다.  
#[allow(dead_code)]
fn deadlock() {
    println!("1");
    let  a = std::sync::Mutex::new(1);
    println!("2");
    let mut g = a.lock().unwrap();
    println!("3");
    *g += 1;
    println!("4");
    // std::mem::drop(g);
    let mut g = a.lock().unwrap();
    println!("5");
    *g += 1;
    println!("6");

}