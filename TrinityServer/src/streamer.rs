// A file intended for audio multicasting and live streaming
// Trinitypeer, 2025, by Trinitycore

use std::sync::Arc;
use actix_web::Responder;
use webrtc::peer_connection::RTCPeerConnection;
use dashmap::DashMap;
use dashmap::mapref::one::Ref;

use actix_web::HttpResponse;
use tokio::time::sleep;

use crate::server::stream;



// A structure to store the stream data
// There is connection with WebRTC to make sure we have our person saved
// And will not do everytime a new one.
// Also, there are streamer id and stream name to better save the data
// And make sure the code is clean, while still performant well enough

pub struct Stream {
    streamer_id: usize,
    stream_name: String,
    connection: Option<Arc<RTCPeerConnection>>,
    current_chunk: Vec<u8>,
}

impl Stream {
    // New stream creation:
    
    pub fn new(streamer_id: usize, stream_name: String, connection: 
                                                    Option<Arc<RTCPeerConnection>>) -> Self {
        Stream {
            streamer_id,
            stream_name,
            connection,
            current_chunk: vec![1, 2, 3, 4],
        }
    }

    pub fn load_chunk(&mut self, chunk: Vec<u8>) {
        self.current_chunk = chunk;
    }

    pub fn get_chunk(&self) -> Vec<u8> {
        self.current_chunk.clone()
    }
}


// A structure to store all the active streams
// Represents a single datatype with an impl methods
// to simplify the code for maintaining the streams

pub struct ActiveStreams {
    streams: Arc<DashMap<String, Stream>>,
}

impl ActiveStreams {
    // IMPORTANT: For any contributor: 
    // Shard amount MUST be a power of 2, in case it is not
    // The DashMap will panic right away afteer creating the stream engine
    // Recommended value is 256 or 512 for the best performance
    // In case the app would grow to very big (100,000+) users
    // Still, it is better to consider the amount of the shards now to make it
    // scalable in the future and make sure the code is performant enough from 
    // the start

    pub fn new(shard_amount : usize) -> Self {
        ActiveStreams {
            streams: Arc::new(DashMap::with_shard_amount(shard_amount)),
        }
    }



    // A method to remove an existing stream, which was already stopped
    // By the user either by system considerations to save the resources

    pub fn remove_stream(&self, stream_id: String) {
        self.streams.remove(&stream_id);
    }



    // A method to add a new stream to the active streams
    // This one normally should be called outside of this file
    // And should be called by the main controller of the streams
    
    pub fn add_stream(&self, c_stream: Stream) {
        self.streams.insert(c_stream.stream_name.clone(), c_stream);
    }



    // Getting the stream by its name
    //pub fn get_stream(&self, stream_name: &str) -> Option<&Stream> {
    //    self.streams.get(stream_name).map(|r| r.value())
    //} 
}

// A function to perform the stream
// This one is called by the main controller of the streams

async fn perform_stream(stream_list: &ActiveStreams, stream_name: String) -> impl Responder {
    // Getting the stream by its identifier
    // let current_stream = stream_list.get_stream(stream_name.clone());
    
    // If the stream is not found, do nothing and end the function
    // if current_stream.is_none() {
    //    return HttpResponse::NotFound().body("Stream not found")
    //}

    //let current_stream = current_stream.unwrap();

    let async_stream_thread = async_stream::stream! {
        loop {
            let chunk = Vec::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
            yield Ok::<_, actix_web::Error>(actix_web::web::Bytes::from(chunk));
            sleep(std::time::Duration::from_secs(2)).await;        
        }
    };

    HttpResponse::Ok()
            .content_type("audio/flac").streaming(async_stream_thread)
}