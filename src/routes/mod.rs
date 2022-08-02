// pub mod author_routes;
// pub use author_routes::author_routes;

// pub mod book_routes;
// pub use book_routes::book_routes;

use axum::{routing::get, Router};

use crate::controllers::{author_controller, book_controller};

pub fn author_routes() -> Router {
    let router = Router::new().route(
        "/",
        get(author_controller::get_authors_handler).post(author_controller::create_author_handler),
    );

    router
}

pub fn book_routes() -> Router {
    let router = Router::new().route(
        "/",
        get(book_controller::get_books_handler).post(book_controller::create_book_handler),
    );

    router
}
