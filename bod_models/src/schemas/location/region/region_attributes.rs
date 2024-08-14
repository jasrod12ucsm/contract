use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Debug,Clone)]
pub struct RegionAttributes {
    pub code: String,
    #[serde(rename = "fullName")]
    pub full_name: String,
    #[serde(rename = "countryId")]
    pub country_id: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
}


