use axum::{routing::get, Router};

use crate::controllers::book_controller;

pub fn book_routes() -> Router {
    let router = Router::new().route(
        "/",
        get(book_controller::get_books_handler).post(book_controller::create_book_handler),
    );

    router
}
