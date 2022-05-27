pub use jsonwebtoken::errors;
use jsonwebtoken::errors::Result;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub exp: usize,
    pub email: String,
    pub username: String,
    pub verified_email: bool,
    pub id: String,
}

/// create a new JWT token
pub fn create_jwt_token<T: Serialize>(private_key: &[u8], claims: T) -> Result<String> {
    encode(
        &Header::new(Algorithm::RS256),
        &claims,
        &EncodingKey::from_rsa_pem(private_key)?,
    )
}

/// verify JWT token correctness
pub fn verify_token<T: DeserializeOwned>(token: String, public_key: &[u8]) -> Result<TokenData<T>> {
    decode::<T>(
        token.as_str(),
        &DecodingKey::from_rsa_pem(public_key)?,
        &Validation::new(Algorithm::RS256),
    )
}
