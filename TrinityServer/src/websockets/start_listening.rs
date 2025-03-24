// Импортируем необходимые элементы из библиотеки Axum:
// - routing::get — для обработки GET-запросов
// - Router — маршрутизатор для HTTP/WebSocket маршрутов
use axum::{routing::get, Router};

// Стандартный тип, представляющий IP-адрес + порт (например, 127.0.0.1:3000)
use std::net::SocketAddr;

// TcpListener из Tokio (из города, нахуй. Уажайтее его) — асинхронный TCP-сервер
use tokio::net::TcpListener;

// TraceLayer — middleware из tower_http, добавляет логирование запросов
// Note: middleware - не знаю, нужно ли оно будет нам,
//                  но это хорошая практика при создании боьших проектов
use tower_http::trace::TraceLayer;

// Импортируем обработчик WebSocket из модуля listening
use super::listening::ws_handler;


pub async fn run_server() {

    // Создаём маршрутизатор с одним GET-маршрутом "/ws", который обрабатывает
    //                                          WebSocket-подключения через HTTP-запросы
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .layer(TraceLayer::new_for_http());

    // Указываем адрес, на котором будет работать сервер (localhost:3000)
    // TODO: Перенести в конфиг и обсудить порт
    //         (хотя хз зачем, но вдруг кто-то из нас запускает сразу несколько локальных серверов)
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Сервер слушает на ws://{}", addr);

    // Создаём асинхронный TCP-сервер, привязанный к этому адресу
    let listener = TcpListener::bind(addr).await.unwrap();

    // Запускаем сервер Axum, передаём ему TCP listener и маршруты
    axum::serve(listener, app).await.unwrap();

}
