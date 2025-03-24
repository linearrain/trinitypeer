// TODO1: Тут всё переделать нахуй
// TODO: но сначала решить, какие будет эндпоинты,
//                              хотя бы на начало, потом можно будет добавить новые)

// Обрабатывает входящие текстовые сообщения от клиента
pub fn match_message(text: &str) -> &'static str {
    match text {
        "Hello" => "Hi there!",
        "How are you?" => "I'm fine, thank you!",
        _ => "I don't understand.",
    }
}