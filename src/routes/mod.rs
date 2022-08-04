// pub mod author_routes;
// pub use author_routes::author_routes;

// pub mod book_routes;
// pub use book_routes::book_routes;

use axum::{
    routing::{delete, get, put},
    Router,
};

use crate::controllers::{author_controller, book_controller};

pub fn author_routes() -> Router {
    Router::new()
        .route(
            "/",
            get(author_controller::get_authors_handler)
                .post(author_controller::create_author_handler),
        )
        .route("/:id", get(author_controller::get_author_handler))
        .route("/:id/update", put(author_controller::update_author_handler))
        .route(
            "/:id/delete",
            delete(author_controller::delete_author_handler),
        )
        .route(
            "/:id/books",
            get(author_controller::get_author_books_handler),
        )
}

pub fn book_routes() -> Router {
    Router::new()
        .route(
            "/",
            get(book_controller::get_books_handler).post(book_controller::create_book_handler),
        )
        .route("/:id", get(book_controller::get_book_handler))
        .route("/:id/update", put(book_controller::update_book_handler))
        .route("/:id/delete", delete(book_controller::delete_book_handler))
        .route(
            "/:id/:author_id",
            get(book_controller::get_books_from_author_handler),
        )
}
