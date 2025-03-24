// Импортируем необходимые элементы из библиотеки Axum:
// - Message — представляет текстовое/бинарное сообщение
// - WebSocket — объект для общения по WebSocket
// - WebSocketUpgrade — обрабатывает Upgrade с HTTP на WebSocket
// - IntoResponse — преобразует ответ в формат, понятный Axum
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};

// futures_util — утилита для асинхронных стримов
// StreamExt — добавляет метод .next() для чтения сообщений из WebSocket
use futures_util::StreamExt;


// Импортируем функцию обработки сообщений из модуля message_handler
use super::message_handler::match_message;


// Функция, вызываемая при обращении к маршруту "/ws"
// Принимает запрос на WebSocket (Upgrade) и передаёт соединение в `handle_socket`
pub async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}


// Обработка подключённого WebSocket-клиента
// Здесь происходит "прослушивание" и реакция на входящие сообщения
async fn handle_socket(mut socket: WebSocket) {

    // Главный цикл прослушки сообщений от клиента, пока клиент не отключится
    while let Some(Ok(msg)) = socket.next().await {

        // TODO1: переделать получаемые сообщение

        // Для теста сделано, чтобы сервер принимал только текстовый тип сообщений
        if let Message::Text(text) = msg {
            println!("Клиент отправил: {}", text);

            // Получаем ответ от функции обработки сообщений
            let response = match_message(&text);

            // Отправляем ответ клиенту обратно через WebSocket
            if socket.send(Message::Text(response.into())).await.is_err() {
                // Если не удалось отправить — логируем ошибку и завершаем соединение
                eprintln!("Ошибка при отправке сообщения");
                return;
            }
        }
    }
}