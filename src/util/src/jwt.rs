use jsonwebtoken::{encode, decode, Algorithm, EncodingKey, Header, DecodingKey, Validation, TokenData};
use jsonwebtoken::errors::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    exp: usize,
}

pub struct CreateToken {
    claims: Claims
}

impl CreateToken {
    pub fn new(exp: usize) -> CreateToken {
        CreateToken {
            claims: Claims {
                exp,
            },
        }
    }

    pub fn create_jwt_token(&self, private_key: &[u8]) -> Result<String> {
        Ok(
            encode(
                &Header::new(Algorithm::RS256),
                &self.claims,
                &EncodingKey::from_rsa_pem(private_key)?,
            )?
        )
    }
}

pub fn verify_token(token: String, public_key: &[u8]) -> Result<TokenData<Claims>> {
    Ok(
        decode::<Claims>(token.as_str(), &DecodingKey::from_rsa_pem(public_key)?, &Validation::new(Algorithm::RS256))?
    )
}
