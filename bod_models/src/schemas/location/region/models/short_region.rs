use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::region_with_id::RegionWithId;

#[derive(Debug, Clone, Serialize, Deserialize, Builder,Default)]
#[builder(setter(into), build_fn(validate = "Self::validate"))]
pub struct ShortRegion {
    pub code: String,
    #[serde(rename = "fullName")]
    pub full_name: String,
    #[serde(rename = "countryId")]
    pub country_id: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
}

impl ShortRegionBuilder {
    fn validate(&self) -> Result<(), String> {
        if self.code.is_none() {
            return Err("code is required".into());
        }
        if self.full_name.is_none() {
            return Err("full_name is required".into());
        }
        if self.country_id.is_none() {
            return Err("country_id is required".into());
        }
        if self.is_active.is_none() {
            return Err("is_active is required".into());
        }
        Ok(())
    }

    pub fn build_partial_update(&self) -> bson::Document {
        let mut doc = bson::Document::new();
        if let Some(code) = &self.code {
            doc.insert("code", code);
        }
        if let Some(full_name) = &self.full_name {
            doc.insert("fullName", full_name);
        }
        if let Some(country_id) = &self.country_id {
            doc.insert("countryId", country_id);
        }
        if let Some(is_active) = &self.is_active {
            doc.insert("isActive", is_active);
        }
        doc
    }
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