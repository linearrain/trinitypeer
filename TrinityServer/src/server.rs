// Server file, basically routes the client where needed
// Also, routes the people from the stream_id to the function
// Which is processing the stream in streamer.rs file

use actix_web::{web, App, HttpServer, HttpResponse};
use crate::streamer::{perform_stream, ActiveStreams};
use actix_web::Responder;

use log::{error, info, warn};
use pretty_env_logger;

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
            .route("/stream/{id}", web::get().to(stream))
    })
    .bind(("192.168.122.1", 13412))?
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
async fn create_stream(streamname :web::Path<String>, stream_list: web::Data<ActiveStreams>) -> HttpResponse {
    let streamname = streamname.into_inner(); 
    let stream = crate::streamer::Stream::new(0, streamname.clone(), None);
    if stream_list.add_stream(stream).await.is_err() {
        error!("Stream with name {} already exists", streamname);
        return HttpResponse::BadRequest().body(format!("Stream with name {} already exists", streamname));
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
