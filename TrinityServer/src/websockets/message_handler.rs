// TODO1: Тут всё переделать нахуй
// TODO: но сначала решить, какие будет эндпоинты,
//                              хотя бы на начало, потом можно будет добавить новые)

use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct LoginMessage {
    #[serde(rename = "type")]
    msg_type: String,
    username: String,
    password: String,
}

pub fn match_message(text: &str) -> String {
    // Пробуем распарсить входящее сообщение как JSON логина
    match serde_json::from_str::<LoginMessage>(text) {
        Ok(login) if login.msg_type == "login" => {
            if login.username == "admin" && login.password == "1234" {
                "ok".to_string()
            } else {
                json!({
                    "status": "error",
                    "message": "Неверный логин или пароль"
                })
                .to_string()
            }
        }
        _ => {
            // Если не удалось распарсить, или неизвестный формат
            json!({
                "status": "error",
                "message": "Неверный формат сообщения"
            })
            .to_string()
        }
    }
}
