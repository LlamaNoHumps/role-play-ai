use crate::database::{Database, models};
use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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

#[derive(Clone)]
pub struct Auth {
    database: Arc<Database>,
}

impl Auth {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
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

    pub async fn verify(&self, token: &str) -> Result<models::users::Model> {
        match unsafe_decode_token(token) {
            Ok(claims) => match self.database.get_user(&claims.username).await {
                Ok(user) => match verify_token(token, &user.jwt_secret) {
                    Ok(_verified_claims) => Ok(user),
                    Err(_) => Err(anyhow::anyhow!("Invalid token signature")),
                },
                Err(_) => Err(anyhow::anyhow!("User not found")),
            },
            Err(_) => Err(anyhow::anyhow!("Invalid token format")),
        }
    }
}

fn verify_token(token: &str, user_secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(user_secret.as_ref()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

fn unsafe_decode_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let mut validation = Validation::default();
    validation.insecure_disable_signature_validation();

    let token_data = decode::<Claims>(token, &DecodingKey::from_secret("".as_ref()), &validation)?;
    Ok(token_data.claims)
}
