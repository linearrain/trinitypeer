use std::env;

use actix::fut::{future::result, ready};
use dotenv::dotenv;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use log::{error, info};
use serde::Deserialize;
use sqlx::FromRow;
use futures_util::future::Ready; 
use actix_web::{dev::Payload, error::ErrorUnauthorized, Error, FromRequest, HttpRequest, HttpResponse};
use webrtc::media::{audio::buffer::info, io::ResetFn};

use crate::auth_logic::jwt_functions::decode_jwt;

use super::jwt_functions::Claims;


#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct RegistrationRequest {
    pub username: String,
    pub nickname: String,
    pub profile_pic_path: String,
    pub password: String
}

// This structure is using by functions which require to find user from database, or register new user.
#[derive(FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub nickname: String,
    pub profile_pic_path: String,
    pub password_hash: String,
}
    
// Structure which will be send by client to server, with access token
#[derive(FromRow)]
pub struct AuthenticatedUser {
    pub username: String
}

// Function for validation and checking token 
// (runs automaticly when function with parameter type of AuthenticatedUser is running)
impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<AuthenticatedUser, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        dotenv().ok();
        let auth_header = req.headers().get("Authorization");

        if let Some(header_value) = auth_header {
       
            if let Ok(auth_str) = header_value.to_str() {
                if auth_str.starts_with("Bearer "){
                    let token = &auth_str[7..];

                    let result = decode_jwt(&token);

                    info!("Token: {:?}", token);

                    if let Ok(data) = result {
                        let username = data.sub;
                        return ready(Ok(AuthenticatedUser { username }));
                    }
                }
            }
        }    

        ready(Err(actix_web::error::ErrorUnauthorized("Invalid or missing token")))   
    }
    
}