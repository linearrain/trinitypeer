// Server file, basically routes the client where needed
// Also, routes the people from the stream_id to the function
// Which is processing the stream in streamer.rs file

use std::env;

use actix_web::{body, error::ErrorUnauthorized, http, web, App, FromRequest, HttpRequest, HttpResponse, HttpServer};
use argon2::password_hash::{self, rand_core::impls};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde_json::json;
use uuid::Error;
// use jsonwebtoken::crypto::verify;
use webrtc::media::audio::buffer::info;
use crate::{auth_logic::{jwt_functions::{decode_jwt, Claims}, models::{AuthenticatedUser, 
    RegistrationRequest, User}}, db::init_db, streamer::{perform_stream, ActiveStreams}};
use actix_web::Responder;

use log::{error, info, warn};
use pretty_env_logger;

// Import of model for authentication request
use crate::auth_logic::models::LoginRequest;

// Import of library to verify and hash passwords
use bcrypt::{verify, hash};

// Import of function which creates jwt token after successful authorization
use crate::auth_logic::jwt_functions::create_jwt;

// The main function for manipulating the server
// There are some main routers which users can use for their needs
// This server is called in the main function right from the start

pub async fn launch_server(stream_list : ActiveStreams, fragment_len: u8) 
                                                     -> std::io::Result<()> {
    // Create a new instance of actix-web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(stream_list.clone()))
            .service(index)
            .service(create_stream)
            .service(load_chunk_to_srv)
            .service(login)
            .service(user_data)
            .service(protectedArea)
            .service(register)
            .service(refreshToken)
            .route("/stream/{id}", web::get().to(stream))
    })
    .bind(("10.10.12.246", 13412))?
    .run()
    .await
}

// The main page of the server
// Currently, there is no information on it, however 
// It will become the main page of the streaming service

#[actix_web::get("/")]
async fn index () -> HttpResponse {
    HttpResponse::Ok().body("mainpg")
}

// The function which is called when the streamer is pushed new chunk to the server
// In most cases, it updates in around 1-2 secs giving the acceptable latency 
// And great experience. The time may vary depending on the internet quality
// (which is currently not implemented yet, but will be in the future)

#[actix_web::post("/load_chunk/{stream_id}")]
async fn load_chunk_to_srv(stream_id: web::Path<String>, 
                           stream_list: web::Data<ActiveStreams>,
                           chunk: web::Json<Vec<u8>>) -> HttpResponse {
    let stream_id = stream_id.into_inner();
    let mut stream = stream_list.get_stream_ref_mut(&stream_id);

    info!("Someone is trying to load a chunk to the stream ID: {:?}", stream_id);

    // In case the reference mut is found, the stream is found as well, so the streamer is
    // Sending the chunks to a valid stream
    // Otherwise the stream is not found, so pushing of the chunk is not possible

    if let Some(mut s) = stream {
        info!("The chunk is loading into the stream");
        s.load_chunk(chunk.into_inner()).await;
        HttpResponse::Ok().body(format!("Chunk loaded to stream ID: {:?}", stream_id))
    } else {
        warn!("Stream ID: {:?} not found", stream_id);
        HttpResponse::NotFound().body(format!("Stream ID: {:?} not found", stream_id))
    }
}

// The stream creation function, one of the main routes here
// It creates a new stream with the given name and ID

#[actix_web::post("/create_stream/{streamname}")]
async fn create_stream(streamname :web::Path<String>, 
                       stream_list: web::Data<ActiveStreams>) -> HttpResponse {
    let streamname = streamname.into_inner(); 
    let stream = crate::streamer::Stream::new(0, streamname.clone(), None);
    if stream_list.add_stream(stream).await.is_err() {
        error!("Stream with name {} already exists", streamname);
        return HttpResponse::BadRequest()
                .body(format!("Stream with name {} already exists", streamname));
    }

    HttpResponse::Ok().body(format!("Stream created with ID: {:?}", streamname))
}

async fn stream(stream_id: web::Path<String>, active_streams: web::Data<ActiveStreams>,
                                             ) -> impl Responder {
    let stream_id = stream_id.into_inner();

    // The fragment length is the length of the chunk which is sent to the user

    let fragment_len = 1; 

    // Perform the streaming operation
    // This function is defined in the streamer.rs file in the case of wondering
    perform_stream(active_streams, stream_id, fragment_len).await
}

/*
async fn get_10_active_streams() -> impl Responder {
    // This function is needed to get the current
    // It wil return 10 random currently going streams


}
*/


// This function is responsible for returning user new access token.
#[actix_web::post("/refresh")]
async fn refreshToken(req: HttpRequest) -> impl Responder {
    if let Some(refresh_token_cookie) = req.cookie("refresh_token") {
        let refresh_token = refresh_token_cookie.value();
        
        let result = decode_jwt(&refresh_token);

        info!("Refresh token was decoded successfully");

        if let Ok(data) = result {
            let username = data.sub;
            let access_token = match create_jwt(&username, "access") {
                Ok(token) => token,
                Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
            };            

            info!("New access token was sent");    
            HttpResponse::Ok().json(json!({"access_token": access_token}));
        }

        HttpResponse::InternalServerError().body("Something went wrong | this error on stage of debug")
    } else {
        return HttpResponse::Unauthorized().body("No refresh token provided");
    }
}

// This function is used for testing a authoriation function from the client side
// Function takes ueser token as parameter and after validation decides to give
// access to the protected area or not
#[actix_web::get("/protected")]
async fn protectedArea(user: AuthenticatedUser) -> impl Responder {
    // let client_token = decode_jwt(&token);
    
    HttpResponse::Ok().body("Welcome to protected area!")
}


// Function to perform registration of user.
#[actix_web::post("/register")]
async fn register(req: web::Json<RegistrationRequest>) -> impl Responder {
    let username = req.username.to_string();
    let nickname = req.nickname.to_string();
    let profile_pic_path = req.profile_pic_path.to_string();
   
    let password_hash_result = hash(req.password.to_string(), 12);

    let password_hash = match password_hash_result {
        Ok(hash) => hash,
        Err(e) => {
            return HttpResponse::InternalServerError()
                    .body(format!("Password hashing failed: {}", e));
        }
    };
    
    let pool = init_db().await;

    match pool {
        Some(pool) => {
            // Checking if user already exists
            let user = sqlx::query_as::<_, User>(
                "SELECT name 
                FROM users WHERE name = $1")
                .bind(&username)
                .fetch_optional(&pool)
                .await;

            match user {
                Ok(Some(user)) => {
                    // User is found. Returning response with 409 status code
                    return HttpResponse::Conflict().body("User already exists!"); 
                },
                Ok(None) => {
                    // User not found, proceed to insert
                    let result = sqlx::query(
                        "INSERT INTO users (name, nickname, profile_pic_path, password_hash)
                        VALUES ($1, $2, $3, $4);")
                        .bind(&username)
                        .bind(&nickname)
                        .bind(&profile_pic_path)
                        .bind(&password_hash)
                        .execute(&pool) 
                        .await;

                    match result {
                        Ok(_) => {
                            // User successfully created
                            return HttpResponse::Created().body("User created successfully!");
                        },
                        Err(e) => {
                            // In case of any database errors
                            return HttpResponse::InternalServerError()
                                    .body(format!("Server error: {}", e));
                        }
                    }
                },
                Err(e) => {
                    // Error during query execution
                    return HttpResponse::InternalServerError()
                            .body(format!("Server error: {}", e));
                }
            }
        },
        None => {
            // If the database connection fails
            return HttpResponse::InternalServerError().body("Failed to connect to the database.");
        }
    }
}


// This function is used to authenticate user by their credentails 
// (username and password) and then returns token
#[actix_web::post("/login")]
async fn login(req: web::Json<LoginRequest>) -> impl Responder {
    info!("Login request received");
    
    let username = req.username.to_string();
    let password = req.password.to_string();

    // Basic validation for values received on input
    // TODO: add validation by REGEX from sql injections
    if(username.len() <= 3 || password.len() <= 3){
        error!("User credentials is not in valid form!");
        HttpResponse::InternalServerError().body("Worng credentials!");
    }

    // Creates session of communication with database
    let pool = init_db().await;

    // Looks for user in database
    match pool {
        Some(pool) => {

            // Query to enter in variable user data from database
            let user = sqlx::query_as::<_, User>(
                "SELECT id, 
                        name, 
                        nickname, 
                        profile_pic_path, 
                        password_hash 
                FROM users WHERE name = $1")
                .bind(&username)
                .fetch_optional(&pool) 
                .await;

            match user {
                Ok(Some(user)) => {
                    // User is found. Starts checking on correct password.                    
                    let password_hash = user.password_hash;
                    let password_is_correct = verify(&password, &password_hash)
                        .unwrap_or(false);
                    if password_is_correct {
                        let access_token = match create_jwt(&username, "access") {
                            Ok(token) => token,
                            Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
                        };
                        
                        let refresh_token = match create_jwt(&username, "refresh") {
                            Ok(token) => token,
                            Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
                        };

                        info!("Login successful. Acess Token: {:?}", access_token);
                        info!("Refresh Token: {:?}", refresh_token);
                        
                        return HttpResponse::Ok().json(json!({
                            "access_token": access_token,
                            "refresh_token": refresh_token
                        }));                        
                    } else {
                        error!("Incorrect Password");
                    }
                },
                Ok(None) => {
                    error!("User not found");
                    HttpResponse::InternalServerError().body("User not found!");
                },
                Err(e) => {
                    HttpResponse::InternalServerError().body(format!("Server error: {}", e));
                }
            }
        },
        None => {
            HttpResponse::InternalServerError().body("Authentication process failed");
        }
    }

    HttpResponse::Ok().body("Logged in!")
}

// This function is used to get user data
// It will be used to get user data from the database
// and return it to the client
#[actix_web::get("/user_data")]
async fn user_data(user: AuthenticatedUser) -> impl Responder {
    info!("User data request received");
    let pool = init_db().await;
    info!("Debug");
    match pool {
        Some(pool) => {
            // Query to enter in variable user data from database
            info!("Trying to find user by nmae {}", user.username);
            let mut user = sqlx::query_as::<_, User>(
                "SELECT id, 
                        name, 
                        nickname, 
                        profile_pic_path,
                        password_hash 
                FROM users WHERE name = $1")
                .bind(&user.username.to_string())
                .fetch_optional(&pool) 
                .await;

            if let Ok(Some(user)) = &user {
                info!("Debug trying to get user by name: {}", user.name);
            } else {
                error!("Failed to retrieve user or user not found");
            }
            
            match user {
                Ok(Some(mut user)) => {
                    info!("User data found: {}", user.name);
                    info!("Token is valid, user data is sending in json");
                    user.password_hash = String::new(); 
                    return HttpResponse::Ok().json(user);
                },
                Ok(None) => {
                    error!("User not found");
                    return HttpResponse::InternalServerError().body("User not found!");
                },
                Err(e) => {
                    error!("Error while fetching user data: {}", e);
                    return HttpResponse::InternalServerError().body(format!("Server error: {}", e));
                }
            }
        },
        None => {
            return HttpResponse::InternalServerError().body("Failed to connect to the database.");
        }
    }
}