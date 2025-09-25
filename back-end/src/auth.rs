use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i32,
    pub username: String,
    pub exp: i64,
}

impl Claims {
    pub fn new(user_id: i32, username: String) -> Self {
        let expiration = Utc::now() + Duration::days(7);
        Self {
            user_id,
            username,
            exp: expiration.timestamp(),
        }
    }
}

pub fn create_token(
    user_id: i32,
    username: String,
    user_secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims::new(user_id, username);
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(user_secret.as_ref()),
    )
}

pub fn verify_token(token: &str, user_secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(user_secret.as_ref()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

pub fn unsafe_decode_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let mut validation = Validation::default();
    validation.insecure_disable_signature_validation();

    let token_data = decode::<Claims>(token, &DecodingKey::from_secret("".as_ref()), &validation)?;
    Ok(token_data.claims)
}
