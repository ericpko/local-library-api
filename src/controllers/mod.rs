pub mod author_controller;
pub mod book_controller;

// API index handler
pub async fn api_index_handler() -> String {
    String::from("API Index")
}
