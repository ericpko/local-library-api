use axum::{routing::get, Router};

use crate::controllers::author_controller;

pub fn author_routes() -> Router {
    let router = Router::new().route(
        "/",
        get(author_controller::get_authors_handler).post(author_controller::create_author_handler),
    );

    router
}
