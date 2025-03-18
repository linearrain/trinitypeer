# Proposal from Trinity Core
## Kostiantyn Zaitsev // Yaroslav Marochok // Vitalii Petushynskyi

# Core Idea

## Concept: 

Trinitypeer - Efficient and memory/panic-safe Real-Time WebSocket streaming service, 
which uses P2P connection to break the boundaries of the traditional streaming 
and make sure the speed will raise even higher, then the current ones and likely 
will open some perspectives on making the data transfer faster for the other massive 
streaming services without a huge drawbacks to business and technology change.

WebSockets are choosen as an alternative approach to the traditional HTTP-streaming,
as it is faster for real-time data and streaming can be used with FLAC format, which 
is better for the audio streaming, than the MP3, which heavily compresses the sound 
(especially in the high frequencies in music such as Metal or Loud Electronic, such 
as Dubstep / Hardcore / Hardstyle and bass-related genres like Trap). The new technology 
do not only brings the speed, but also allows to make the streaming more efficient and 
at the same time high-quality, which is a huge advantage for the music streaming services 
in 2020s, when the quality of the sound is a key factor for the user experience and business 
concurrency.

## Technologies:

Obviosly, ```Rust``` programming language is the core of our service, but some 
libraries must be used:

```tokio``` - as the app is server-dependent, the async technology must be used 
and tokio is one, that effective and lightweight for the server programming

```websocket``` - a library for opening the websocket connection, which will 
effectively handle the connections and support bidirectional communication, 
which will simplify the structure and improve the overall efficiency, compared 
to the HTTP-streaming, popular nowadays for various purposes.

```serde``` - a nice crate for serializing the objects to binary / bytes / json 
to later use them for data passing on the current side and its recreation on the
 other side of the communication.

```axum``` - incredibly fast file streaming.

```log``` - for logging the events and errors, which will help to debug the code 
and report the status of the service to the user.



## Front-End:

```Figma``` - for designing the app, which will help us to understand, how will it 
look.

```Svelte``` - for creating a very flexible and fast UI for the app, which will be 
better than ```egui``` and ```iced```, as it is more flexible and has finished docs.

```Tauri``` -  with Svelte, a nice Rust back-end framework for connecting both parts
of the app and making it work as a single piece of software.

```Tailwind``` - a possible option in case the optimization of the app styles will 
be needed even further.
