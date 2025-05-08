mod websockets;
mod streamer;
mod audio_coding;
mod server;
mod db;
mod auth_logic;
use dotenv::dotenv;

use log::{error, info, warn};
use pretty_env_logger;

use std::fs::File;
use std::io::Read;



#[tokio::main]
async fn main() {
    // Initialize the logger
    pretty_env_logger::env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();

    // Initialize the DashMap, which stroes all the running streams
    let streams = streamer::ActiveStreams::new(256);

    // Initialize the fragment length for the live stream
    let fragment_len = 1;



    // Initialize the HTTP Server
    server::launch_server(streams, fragment_len).await.expect("Failed to start server");
}