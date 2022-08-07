use crate::models::book::Book;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Result as AxumResult},
    Extension, Json,
};
use bson::doc;
use futures::stream::TryStreamExt;
use mongodb::{options::FindOptions, Database};

pub async fn get_books_handler(Extension(db): Extension<Database>) -> AxumResult<Json<Vec<Book>>> {
    let books_collection = db.collection::<Book>("books");

    let find_options = FindOptions::builder().sort(doc! {"title": 1}).build();
    let books_cursor = books_collection.find(None, find_options).await.unwrap();
    let books = books_cursor.try_collect().await.unwrap();

    Ok(Json(books))
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
