use axum::{http::StatusCode, Json};

use crate::models::author::Author;

pub async fn get_authors_handler(Json(_payload): Json<Author>) -> Result<Json<Author>, StatusCode> {
    let dummy = Author::new(None, "blah".to_string(), "blah".to_string(), None, None);
    Ok(Json(dummy))
}

pub async fn create_author_handler(payload: Json<Author>) -> Result<Json<Author>, StatusCode> {
    let payload = payload.0;
    Ok(Json(payload))
}
