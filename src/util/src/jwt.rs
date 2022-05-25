pub use jsonwebtoken::errors;
use jsonwebtoken::errors::Result;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub exp: usize,
    pub email: String,
    pub username: String,
}

/// create a new JWT token
pub fn create_jwt_token(private_key: &[u8], claims: Claims) -> Result<String> {
    encode(
        &Header::new(Algorithm::RS256),
        &claims,
        &EncodingKey::from_rsa_pem(private_key)?,
    )
}

/// verify JWT token correctness
pub fn verify_token(token: String, public_key: &[u8]) -> Result<TokenData<Claims>> {
    decode::<Claims>(
        token.as_str(),
        &DecodingKey::from_rsa_pem(public_key)?,
        &Validation::new(Algorithm::RS256),
    )
}
