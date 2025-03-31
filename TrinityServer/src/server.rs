// Server file, basically routes the client where needed
// Also, routes the people from the stream_id to the function
// Which is processing the stream in streamer.rs file

use actix_web::{web, App, HttpServer, HttpResponse};

pub async fn launch_server() -> std::io::Result<()> {
    // Create a new instance of actix-web server

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(stream)
    })
    .bind(("127.0.0.1", 13412))?
    .run()
    .await
}

#[actix_web::get("/")]
async fn index () -> HttpResponse {
    HttpResponse::Ok().body("Hello, Січ!")
}

#[actix_web::get("/stream/{stream_id}")]
async fn stream(stream_id: web::Path<String>) -> HttpResponse {
    let stream_id = stream_id.into_inner();
    crate::streamer::perform_streaming(stream_id).await;
}