use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    title: String,
    author: ObjectId,
    summary: String,
    isbn: String,
    genre: Option<Vec<ObjectId>>,
}

impl Book {
    pub fn new(
        id: Option<ObjectId>,
        title: String,
        author: ObjectId,
        summary: String,
        isbn: String,
        genre: Option<Vec<ObjectId>>,
    ) -> Self {
        Self {
            id,
            title,
            author,
            summary,
            isbn,
            genre,
        }
    }
}
