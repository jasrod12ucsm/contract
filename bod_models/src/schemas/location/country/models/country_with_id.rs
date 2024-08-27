use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CountryWithId {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub code: String,
    pub fullname: String,
    pub currency: String,
    #[serde(rename = "currencySymbol")]
    pub currency_symbol: String,
    pub timezones: Vec<String>,
    pub langs: Vec<String>,
     #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
    #[serde(rename = "isActive")]
    pub is_active: bool,
}
