use serde::{Deserialize, Serialize};

use super::region_with_id::RegionWithId;

#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct ShortRegion {
    pub code: String,
    #[serde(rename = "fullName")]
    pub full_name: String,
    #[serde(rename = "countryId")]
    pub country_id: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
}

impl From<RegionWithId> for ShortRegion {
    fn from(value: RegionWithId) -> Self {
        Self {
            code: value.code,
            full_name: value.full_name,
            country_id: value.country_id,
            is_active: value.is_active,
        }
    }
}
