use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use util::jwt::{Claims, create_jwt_token, verify_token};

#[test]
fn create_jwt_token_test() {
    let private_key = fs::read("../../privkey.pem").unwrap();
    let public_key = fs::read("../../pubkey.pem").unwrap();

    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let token = create_jwt_token(private_key.as_slice(), Claims { exp: exp as usize })
        .unwrap();

    verify_token(token.clone(), public_key.as_slice()).unwrap();
}