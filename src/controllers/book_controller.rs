use axum::{
    http::StatusCode,
    response::{IntoResponse, Result},
    Json,
};

use crate::models::book::Book;

pub async fn get_books_handler(Json(payload): Json<Book>) -> Result<Json<Book>, StatusCode> {
    Ok(Json(payload))
}

pub async fn create_book_handler(Json(payload): Json<Book>) -> impl IntoResponse {
    (StatusCode::CREATED, Json(payload))
}

pub async fn get_book_handler() -> impl IntoResponse {
    String::from("NOT IMPLEMENTED")
}

pub async fn delete_book_handler() -> impl IntoResponse {
    (StatusCode::NO_CONTENT, String::from("NOT IMPLEMENTED"))
}

pub async fn update_book_handler() -> impl IntoResponse {
    (StatusCode::NO_CONTENT, String::from("NOT IMPLEMENTED"))
}

pub async fn get_books_from_author_handler() -> impl IntoResponse {
    String::from("NOT IMPLEMENTED")
}
