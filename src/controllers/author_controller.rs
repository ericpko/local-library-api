use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use bson::oid::ObjectId;

use crate::models::author::Author;

pub async fn get_authors_handler() -> Result<Json<Author>, StatusCode> {
    let dummy = Author::new(None, "blah".to_string(), "blah".to_string(), None, None);
    Ok(Json(dummy))
}

pub async fn create_author_handler(
    Json(payload): Json<Author>,
) -> Result<Json<Author>, StatusCode> {
    Ok(Json(payload))
    // (StatusCode::CREATED, Json(message))
}

pub async fn get_author_handler(Path(_author_id): Path<ObjectId>) -> impl IntoResponse {
    // find specific author in the database and return it
    String::from("NOT IMPLEMENTED")
}

pub async fn delete_author_handler() -> impl IntoResponse {
    (StatusCode::NO_CONTENT, String::from("NOT IMPLEMENTED"))
}

pub async fn update_author_handler() -> impl IntoResponse {
    (StatusCode::NO_CONTENT, String::from("NOT IMPLEMENTED"))
}

pub async fn get_author_books_handler() -> impl IntoResponse {
    String::from("NOT IMPLEMENTED")
}
