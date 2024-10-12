use bson::{doc, oid::ObjectId};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(setter(into), build_fn(validate = "Self::validate"))]
pub struct ContentTypeAttributes {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
}

impl ContentTypeAttributesBuilder {
    fn validate(&self) -> Result<(), String> {
        if self.id.is_none() {
            return Err("Name is required".into());
        }
        if self.is_active.is_none() {
            return Err("Is active is required".into());
        }
        if self.is_deleted.is_none() {
            return Err("Is deleted is required".into());
        }
        Ok(())
    }

    pub fn build_partial_update(&self) -> bson::Document {
        let mut doc = doc! {};
        if let Some(id) = &self.id {
            doc.insert("_id", id);
        }
        if let Some(is_active) = &self.is_active {
            doc.insert("isActive", is_active);
        }
        if let Some(is_deleted) = &self.is_deleted {
            doc.insert("isDeleted", is_deleted);
        }
        doc
    }
}