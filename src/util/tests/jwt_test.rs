use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use util::jwt::{CreateToken, verify_token};

#[test]
fn create_jwt_token_test() {
    let private_key = fs::read("../../privkey.pem").unwrap();
    let public_key = fs::read("../../pubkey.pem").unwrap();

    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let token = CreateToken::new((exp.as_secs() + 10) as usize)
        .create_jwt_token(private_key.as_slice())
        .unwrap();

    verify_token(token.clone(), public_key.as_slice()).unwrap();
}