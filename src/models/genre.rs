use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Genre {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
}

impl Genre {
    pub fn new(id: Option<ObjectId>, name: String) -> Self {
        Self { id, name }
    }
}
