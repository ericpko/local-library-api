use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BookInstance {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    book: ObjectId,
    imprint: String,
    #[serde(default = "Status::maintenance")]
    status: Status,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    due_back: DateTime<Utc>,
}

impl BookInstance {
    pub fn new(id: Option<ObjectId>, book: ObjectId, imprint: String, status: Status) -> Self {
        Self {
            id,
            book,
            imprint,
            status,
            due_back: Utc::now(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Available,
    Maintenance,
    Loaned,
    Reserved,
}

impl Status {
    fn maintenance() -> Self {
        Self::Maintenance
    }
}
