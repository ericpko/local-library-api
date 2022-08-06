use axum::Json;
use std::collections::HashMap;

use axum::{response::IntoResponse, Extension};
use bson::Document;
use mongodb::Database;

pub mod author_controller;
pub mod book_controller;

// API index handler
pub async fn api_index_handler(Extension(db): Extension<Database>) -> impl IntoResponse {
    // find the number of documents in each collection
    let mut map = HashMap::new();

    // TODO: handle mongodb error
    for collection_name in db.list_collection_names(None).await.unwrap() {
        let collection = db.collection::<Document>(&collection_name);
        map.entry(collection_name)
            .or_insert(collection.count_documents(None, None).await.unwrap());
    }

    // return a json of the data
    Json(map)
}
