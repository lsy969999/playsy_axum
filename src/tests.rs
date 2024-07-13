pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
}

#[test]
fn test_rand_alphanumeric_code() {
    let code = crate::utils::rand::generate_alphanumeric_code();
    println!("code: {}", code);
}