// A file for encoding / decoding the livestream audio data
// Preffered technology is FLAC, as it is a lossless codec
// It provides the best quality from all the lossless codecs
// And of course it is less bandwidth consuming than the raw PCM data

// Trinitypeer, 2025, by Trinitycore

// A function to convert the Raw PCM data, got from the server
// which actually got it from the client and then then get 
// compressed it straight to the flac format

use flacenc::{self, component::{Stream, BitRepr}, error::{EncodeError, Verify}};
use log::{error, info, warn};
use pretty_env_logger;
use serde::Serialize;

fn encode_pcm_to_flac(pcm_data: &[i32], sample_rate: u32)  -> Option<Vec<u8>> {
    // Setting all the needed settings
    // Channels: the number of audio channels (2 is stereo, please do not touch it)
    // Bits per sample: the number of bits used to represent each sample
    // 16 bits is a better choice for live audio, decent quality and normal bandwidth
    // Sample Rate is the rate at which the audio is sampled, affecting the quality

    let (channels, bits_per_sample, sample_rate) = (2, 16, 44100);    
    let config = flacenc::config::Encoder::default().into_verified();



    // As stated in their documentation, the config must be verified
    // to make sure it is correct and will not cause any problems
    // In case the config is not verified, the program will panic
    // We will prevent it by just returning None and logging the error

    match config {
        Ok(config_verified) => {
            // Source is a converted Vec PCM data
            let source = flacenc::source::MemSource::from_samples(
        pcm_data, channels, bits_per_sample, sample_rate);
            
            // This is our encoder, which will encode the PCM data to FLAC
            let flac_stream = flacenc::encode_with_fixed_block_size(
                &config_verified, source, config_verified.block_size
            );

            match flac_stream {
                Ok(flac_data) => {
                    info!("Encoded the data");

                    // Writing the data to the sink, as it has the method for converting
                    // the data to the Vec<u8> format (as_slice)

                    let mut sink = flacenc::bitsink::ByteSink::new();

                    if flac_data.write(&mut sink).is_err() {
                        error!("Failed to write data to sink");
                        return None;
                    }



                    // If the error was not raised, we can return the data
                    // as a Vec<u8> format

                    Some(sink.as_slice().to_vec())
                }
                Err(e) => {
                    error!("Failed to encode PCM data: {}", e);
                    None
                }
            }
        },
        Err(e) => {
            error!("Failed to verify config: {}", e.1);
            None
        }
    } 
}