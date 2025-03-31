/* Подключаем модуль WebSockets (Блять, какого ещё и папки должны зазываться snake??????),
        в котором лежит вся логика сервера (принятие запросов)  */
mod websockets;
mod streamer;
mod audio_coding;
mod server;

#[tokio::main]
async fn main() {
    server::launch_server().await.expect("Failed to start server");
}