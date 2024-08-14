use bod_models::schemas::location::region::region_attributes::RegionAttributes;

use serde::{Serialize, Deserialize};

pub type AccuRegions = Vec<AccuRegion>;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccuRegion {
    #[serde(rename = "ID")]
    id: String,
    localized_name: String,
    english_name: String,
    level: i64,
    localized_type: Type,
    english_type: Type,
    #[serde(rename = "CountryID")]
    country_id: String,
}


#[derive(Serialize, Deserialize)]
pub enum Type {
    Municipality,
    Region,
}

impl From<AccuRegion> for RegionAttributes{
    fn from(value: AccuRegion) -> Self {
        Self {
            code: value.id,
            full_name: value.localized_name,
            country_id: value.country_id,
            is_active: true,
        }
    }
}