use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use crate::models::_entities::users;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub pid: String,
    pub exp: usize,
}

pub fn generate_token(user: &users::Model) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize + 3600 * 24; // Token valid for 24 hours

    let claims = Claims {
        pid: user.pid.to_string(),
        exp: expiration,
    };

    let token = encode(
        &Header::default(), 
        &claims, 
        &EncodingKey::from_secret(secret.as_ref())
    )?;

    Ok(token)
}

#[derive(Debug, Deserialize)]
pub struct JWT {
    pub claims: Claims,
}
