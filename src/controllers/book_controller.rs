use axum::{http::StatusCode, response::Result, Json};

use crate::models::book::Book;

pub async fn get_books_handler(Json(payload): Json<Book>) -> Result<Json<Book>, StatusCode> {
    Ok(Json(payload))
}

// TODO: try sending StatusCode error without specifying it in the Result<>
pub async fn create_book_handler(Json(payload): Json<Book>) -> Result<Json<Book>> {
    Ok(Json(payload))
}
