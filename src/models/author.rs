use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    first_name: String,
    family_name: String,

    #[serde_as(as = "Option<bson::DateTime>")]
    date_of_birth: Option<chrono::DateTime<chrono::Utc>>,

    #[serde_as(as = "Option<bson::DateTime>")]
    date_of_death: Option<chrono::DateTime<chrono::Utc>>,
}

impl Author {
    pub fn new(
        id: Option<ObjectId>,
        first_name: String,
        family_name: String,
        date_of_birth: Option<chrono::DateTime<chrono::Utc>>,
        date_of_death: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        Self {
            id,
            first_name,
            family_name,
            date_of_birth,
            date_of_death,
        }
    }
}
