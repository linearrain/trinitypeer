use serde::Deserialize;
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
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
    


