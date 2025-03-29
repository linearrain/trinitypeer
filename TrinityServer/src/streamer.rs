// A file intended for audio multicasting and live streaming
// Trinitypeer, 2025, by Trinitycore

use std::sync::Arc;
use webrtc::peer_connection::RTCPeerConnection;
use dashmap::DashMap;
use dashmap::mapref::one::Ref;



// A structure to store the stream data
// There is connection with WebRTC to make sure we have our person saved
// And will not do everytime a new one.
// Also, there are streamer id and stream name to better save the data
// And make sure the code is clean, while still performant well enough

struct Stream {
    streamer_id: usize,
    stream_name: String,
    connection: Arc<RTCPeerConnection>,
}

impl Stream {
    // New stream creation:
    
    fn new(streamer_id: usize, stream_name: String, connection: 
                                                    Arc<RTCPeerConnection>) -> Self {
        Stream {
            streamer_id,
            stream_name,
            connection,
        }
    }
}


// A structure to store all the active streams
// Represents a single datatype with an impl methods
// to simplify the code for maintaining the streams

struct ActiveStreams {
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

    fn new(shard_amount : usize) -> Self {
        ActiveStreams {
            streams: Arc::new(DashMap::with_shard_amount(shard_amount)),
        }
    }



    // A method to remove an existing stream, which was already stopped
    // By the user either by system considerations to save the resources

    fn remove_stream(&self, stream_id: String) {
        self.streams.remove(&stream_id);
    }



    // A method to add a new stream to the active streams
    // This one normally should be called outside of this file
    // And should be called by the main controller of the streams
    
    fn add_stream(&self, stream: Stream) {
        self.streams.insert(stream.stream_name.clone(), stream);
    }



    // Getting the stream by its name
    
    fn get_stream(&self, stream_name: String) -> Option<Ref<'_, String, Stream>> {
        self.streams.get(&stream_name)
    }
}

// A function to perform the stream
// This one is called by the main controller of the streams

async fn perform_stream(stream: Stream) {
    // TODO: understand how WebRTC works and write this function
}