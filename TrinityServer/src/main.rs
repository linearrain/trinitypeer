/* Подключаем модуль WebSockets (Блять, какого ещё и папки должны зазываться snake??????),
        в котором лежит вся логика сервера (принятие запросов)  */
mod websockets;


#[tokio::main]
async fn main() {

    // Запускаем сервер WebSocket-ов
    websockets::start_listening::run_server().await;
}