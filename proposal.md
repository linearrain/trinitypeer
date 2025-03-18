# Proposal from Trinity Core
## Kostiantyn Zaitsev // Yaroslav Marochok // Vitalii Petushynskyi

# Core Idea

## Concept: 

Trinitypeer - Efficient and memory/panic-safe Real-Time WebSocket streaming service, 
which uses WebSocket connection to break the boundaries of the traditional streaming 
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

We are planning to learn how to make complex and big systems with maximally efficient 
and reliable code, which is easy to maintain and grow. The project should be a good 
starting point for a serious practice of internet data streaming and thinking about 
the architecture of these enormous systems.

There may be some chance the audio effects (slow down f. e.) will be added to the
app, but until yet we did not figured out if the suitable library exists for Rust, 
as already does JUCE for C++ and very powerful WebAudioAPI for JavaScript. Therefore, 
this idea is not yet confirmed and may not be supported, that is why it is not included
to the requirements section, but still could be a great deal for the future development.

Despite the project is Rust-based, the front-end part would be better written in Svelte, as
currently there is no suitable UI library for writing a complex and flexible UI in Rust: 
egui is too simple and iced is too complex and not well-documented. Svelte is the only way, 
which is TypeScript-based, so some part of front-end (not back-end) will be written in 
TypeScript to access the API of our server and therefore get the best results we can and by 
that show, how Rust can be used in real-world applications as a powerful brain of the system.

## Requirements:

The service should be able to transmit audio or video data from server after the user 
requests the material to stream. Multiple connections should be successfully handled 
and the server should normally work with the data without the crashes or memory leaks.
The tracks should not be distorted in the way of loosing the quality, so the FLAC or WAV 
/ RAW format should be handled by the code without any issues. The service should react 
fast enough for the user's request, but not instantly: WebSockets are a bit slower for 
establishing the connection, but way too faster for the data transfer, so the main goal 
is to achieve an uninterrupted and fast streaming, rather than the instant connection and 
later loading for 3-4 seconds, which irritates the listener.

The program should be intuitive enough, though some minimal manual will be provided to keep 
the thing simpler for sure. 

Normally, Rust code after compilation is memory-safe and panic-safe, as well as bug-free 
(checked on the experience: the code of one of the team members was impossible to compile 
until the logic was perfect), so this language will help at some point to make sure we 
are not going to crash if the user will try to break the system or make an ordinary thing.

The code should be well-documented and clear enough to read, so the module architecture 
is mandatory. The code will be divided to the small modules, but the key ierarchy is 
front and back-end part, well-separated and connected with the Tauri framework and WebSocket 
API in each of the languages. Some pieces of the code will be written in TypeScript, as 
Svelte is TypeScript-based and the API of the server should be accessed from the front-end, but
all the logic of streaming, getting the requests, processing the wav / flac files and sending 
them is barely Rust-based and will be written in Rust with additional libraries specified **below**.

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
