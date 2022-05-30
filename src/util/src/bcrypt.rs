extern crate bcrypt;

use bcrypt::{hash, verify, BcryptResult, DEFAULT_COST};

pub fn password_hash(password: &String) -> BcryptResult<String> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(hash: String, password: &String) -> BcryptResult<bool> {
    verify(password, hash.as_ref())
}
