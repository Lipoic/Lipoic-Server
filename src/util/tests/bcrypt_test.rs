use util::bcrypt::{password_hash, verify_password};

#[test]
fn bcrypt_test() {
    let password_hash = password_hash(&"123".to_string()).unwrap();
    let verify = verify_password(password_hash, &"123".to_string()).unwrap();
    assert_eq!(true, verify);
}
