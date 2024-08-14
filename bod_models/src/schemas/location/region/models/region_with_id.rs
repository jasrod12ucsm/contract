
use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RegionWithId {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub code: String,
    #[serde(rename = "fullName")]
    pub full_name: String,
    #[serde(rename = "countryId")]
    pub country_id: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
}

impl RegionWithId {
    pub fn new(code: String, full_name: String, country_id: String) -> Self {
        let now = DateTime::now();
        Self {
            id: ObjectId::new(),
            code,
            full_name,
            country_id,
            is_active: true, // Default to true, or change as needed
            created_at: now,
            updated_at: now,
        }
    }
}