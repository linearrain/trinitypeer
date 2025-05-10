use std::{env};
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized, Error};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use dotenv::dotenv;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub tokenType: String
}

// This function creates a JWT token
pub fn create_jwt(uid: &str, token_type: &str) -> Result<String, Error> {
    // Load environmental variables from .env
    dotenv().ok();

    let expiration = match token_type {
        "access" => (Utc::now() + chrono::Duration::minutes(15)).timestamp() as usize,
        "refresh" => (Utc::now() + chrono::Duration::days(30)).timestamp() as usize,
        _ => return Err(ErrorInternalServerError("Invalid token type")),
    };
    
    let claims = Claims {
        sub: uid.to_owned(),
        exp: expiration,
        tokenType: token_type.to_string(),
    };

    let secret = env::var("SECRET_JWT_KEY").expect("Environmental variable isn’t loaded");    

    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .map_err(|e| ErrorInternalServerError(format!("JWT encoding error: {}", e)))
}

// This function decodes jwt tokens
pub fn decode_jwt(token: &str) -> Result<Claims, Error>{
    dotenv().ok();
    let secret = env::var("SECRET_JWT_KEY").expect("Environmental variable isn’t loaded");

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    )
    .map(|data| data.claims)
    .map_err(|err| {
        error!("JWT token decoding error: {}", err);
        ErrorUnauthorized("Invalid token") 
    })
}