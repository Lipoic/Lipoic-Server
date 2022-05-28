pub use jsonwebtoken::errors;
use jsonwebtoken::errors::Result;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::de::DeserializeOwned;
use serde::{Serialize};

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
