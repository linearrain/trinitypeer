// A file intended for audio multicasting and live streaming
// Trinitypeer, 2025, by Trinitycore

use std::sync::Arc;
use std::thread::current;
use actix_web::{web, Responder};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use webrtc::peer_connection::RTCPeerConnection;
use dashmap::DashMap;
use dashmap::mapref::one::{Ref, RefMut};

use actix_web::HttpResponse;

use log::{error, info, warn};

use tokio::time::{interval, Duration};
use tokio::sync::{Notify, RwLock};



// A structure to store the stream data
// There is connection with WebRTC to make sure we have our person saved
// And will not do everytime a new one.
// Also, there are streamer id and stream name to better save the data
// And make sure the code is clean, while still performant well enough

#[derive(Debug)]
pub struct Stream {
    streamer_id: usize,
    stream_name: String,
    connection: Option<Arc<RTCPeerConnection>>,
    current_chunk: Arc<RwLock<Vec<u8>>>,
    change: Arc<Notify>,
}

impl Stream {
    // New stream creation:
    
    pub fn new(streamer_id: usize, stream_name: String, 
               connection: Option<Arc<RTCPeerConnection>>) -> Self {

        
        Stream {
            streamer_id,
            stream_name,
            connection,
            current_chunk: Arc::new(RwLock::new(Vec::new())),
            change: Arc::new(Notify::new()),
        }
    }

    // Push the current streamed chunk
    pub async fn load_chunk(&mut self, chunk: Vec<u8>) {
        *self.current_chunk.write().await = chunk;
    }

    // Get the current chunk
    pub async fn get_chunk(&self) -> Vec<u8> {
        self.current_chunk.read().await.clone()
    }
}


// A structure to store all the active streams
// Represents a single datatype with an impl methods
// to simplify the code for maintaining the streams

#[derive(Clone, Debug)]
pub struct ActiveStreams {
    pub streams: Arc<DashMap<String, Stream>>,
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

    pub async fn remove_stream(&self, stream_id: String) {
        self.streams.remove(&stream_id);
    }



    // A method to add a new stream to the active streams
    // This one normally should be called outside of this file
    // And should be called by the main controller of the streams
    
    pub async fn add_stream(&self, c_stream: Stream) -> Result<(), Box<dyn std::error::Error>> {
        // Check if the stream already exists
        if self.get_stream(&c_stream.stream_name).await.is_some() {
            error!("Stream with name {} already exists", c_stream.stream_name);
            return Err(Box::from(format!("Stream with name {} already exists", c_stream.stream_name)));
        }

        // In case the stream indeed could be created, creating it

        info!("Creating stream with name {}", c_stream.stream_name);
        self.streams.insert(c_stream.stream_name.clone(), c_stream);

        Ok(())
    }



    // Getting the stream by its name
    //pub async fn get_stream(&self, stream_name: &str) -> Option<Stream> {
    //    self.streams.get(stream_name).map(|r| r.value().clone())
    //} 

    // Getting the stream by its name, but as a reference
    pub async fn get_stream(&self, stream_name: &str) -> Option<Ref<String, Stream>> {
        self.streams.get(stream_name)
    }



    // Getting the amount of random streams, which will be used
    // to get random active streams in the HTTP route get_10active_streams

    /*
    pub async fn get_random_streams(&self, amount : usize) -> Vec<&Stream> {
        let mut selection : Vec<&Stream> = Vec::new();

        self.streams.iter().for_each(|stream| {
            selection.push(stream.value().clone());
        }); 


        selection.truncate(amount);
        selection
    }
    */

    // For altering the Stream (for side of the streamer) this function becomes very handy
    // It is a mutable reference to the stream, so it can be changed
    // Preferably, it should change the current chunk of the stream
    // But anything should be acceptable

    pub fn get_stream_ref_mut(&self, stream_name: &str) -> Option<RefMut<String, Stream>> {
        self.streams.get_mut(stream_name)
    }
}

// A function to perform the stream
// This one is called by the main controller of the streams

pub async fn perform_stream(stream_list: web::Data<ActiveStreams>, stream_name: String, fragment_len: u8) -> impl Responder {
    let stream = stream_list.get_stream(&stream_name).await;

    if stream.is_none() {
        warn!("Stream not found");
        return HttpResponse::NotFound().body("Stream not found");
    }

    let mut ticker = interval(Duration::from_millis((fragment_len as u64) * 1000));

    let stream_list = stream_list.clone();


    let async_stream_thread = async_stream::stream! {
    
    let mut prev_chunk_hash : u64 = 0;

    loop {
        ticker.tick().await;

        if let Some(current_stream) = stream_list.get_stream(&stream_name).await {
            let chunk = current_stream.get_chunk().await;

            let current_chunk_hash = hash_vecu8(&chunk);

            if current_chunk_hash == prev_chunk_hash {
                info!("No new chunk to stream");
                continue;
            }

            prev_chunk_hash = current_chunk_hash;            

            yield Ok::<_, actix_web::Error>(actix_web::web::Bytes::from(chunk));
            } 
            else {
                warn!("Stream disappeared during playback");
                break;
            }
        }
    };

    HttpResponse::Ok()
        .append_header(("Connection", "keep-alive"))
        .content_type("audio/flac")
        .streaming(async_stream_thread)
}

fn hash_vecu8(vec : &Vec<u8>) -> u64 {
    let mut hasher = DefaultHasher::new();
    vec.hash(&mut hasher);
    let hash = hasher.finish();

    hash
}
