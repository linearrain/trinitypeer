# Trinitypeer Server

It is a git repo for the server of our social network prototype, which is hosted on AWS currently, but could be launched on your PC as well. No code change is needed, 0.0.0.0 is used.

### 100% of Rust code, no any additional languages used 

# IMPORTANT: Trinitypeer CLIENT

The trinitypeer client has being developed inside of [This Repository](https://github.com/linearrain/trinitypeer-clientside). It was written in Svelte and MORE THAN A HALF in Rust. 

## Functions, implemented in Rust including, but not limited to:

- Capturing the data in one click (used a complex engine described in **audio_capture.rs** file)
    - Was debugged more than for 2 weeks to get working, no good docs were provided to a crate
- Sending the data operatively via HTTP
- Full login / register proccess with secure data transmission
- Streaming capability
- FULL IMPLEMENTATION of the music player, no Javascript used in any part of it
- Buffering for the music player and connection stabilizer
