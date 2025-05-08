use std::{env, error::Error};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use dotenv::dotenv;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

// This function creates a JWT token
pub fn create_jwt(uid: &str) -> Result<String, Box<dyn Error>> {
    // Load environmental variables from .env
    dotenv().ok();
        
    let expiration = (Utc::now().naive_utc() + 
            chrono::naive::Days::new(1)).and_utc().timestamp() as usize;
    let claims = Claims {
        sub: uid.to_owned(),
        exp: expiration
    };

    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(env::var("SECRET_JWT_KEY").expect("Environmental variable isn`t loaded").as_bytes()))
        .map_err(|err| format!("JWT token creation error: {}", err).into())
}

// This function decodes and validates a JWT token
pub fn decode_jwt(token: &str) -> Result<Claims, Box<dyn Error>> {
    // Load environmental variables from .env
    dotenv().ok();
        
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(env::var("SECRET_JWT_KEY").expect("Environmental variable isn`t loaded").as_bytes()),
        &Validation::new(Algorithm::HS512),
    )
    .map(|data| data.claims)
    .map_err(|err| format!("JWT token decoding error: {}", err).into())
}