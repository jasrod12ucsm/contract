use async_trait::async_trait;
use bson::{doc, oid::ObjectId, DateTime};
use derive_builder::Builder;
use mongodb::{options::IndexOptions, results::CreateIndexesResult, Client, IndexModel};
use serde::{Deserialize, Serialize};

use crate::shared::{
    index_functions::IndexFunctions,
    schema::{BaseColleccionNames, Schema},
};

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(setter(into), build_fn(validate = "Self::validate"))]
pub struct ContentType {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
}

impl ContentTypeBuilder {
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
        if self.updated_at.is_none() {
            return Err("Updated at is required".into());
        }
        if self.created_at.is_none() {
            return Err("Created at is required".into());
        }
        Ok(())
    }

    pub fn build_partial_update(&self) -> bson::Document {
        let mut doc = doc! {};
        if let Some(id) = &self.id {
            doc.insert("_id", id);
        }
        if let Some(name) = &self.name {
            doc.insert("name", name);
        }
        if let Some(is_active) = &self.is_active {
            doc.insert("isActive", is_active);
        }
        if let Some(is_deleted) = &self.is_deleted {
            doc.insert("isDeleted", is_deleted);
        }
        if let Some(updated_at) = &self.updated_at {
            doc.insert("updatedAt", updated_at);
        }
        if let Some(created_at) = &self.created_at {
            doc.insert("createdAt", created_at);
        }
        doc
    }
}

pub struct ContentTypeSchema;

impl BaseColleccionNames for ContentType {
    fn get_collection_name() -> &'static str {
        "mst-content-type"
    }

    fn get_database_name() -> &'static str {
        "bod"
    }
}

#[async_trait]
impl Schema for ContentTypeSchema {
    fn get_collection_name(&self) -> &'static str {
        "mst-content-type"
    }

    fn get_database_name(&self) -> &'static str {
        "bod"
    }

    async fn set_indexes(
        &self,
        client: &Client,
    ) -> Result<Option<CreateIndexesResult>, mongodb::error::Error> {
        let collection = client
            .database(self.get_database_name())
            .collection::<ContentType>(self.get_collection_name());
        let mut indexes: Vec<IndexModel> = vec![];
        let unique_name_index = IndexModel::builder()
            .keys(doc! {"name": 1, "isDeleted": 1, "isActive": 1})
            .options(
                IndexOptions::builder()
                    .unique(true)
                    .name("id".to_string())
                    .build(),
            )
            .build();
        indexes.push(unique_name_index);

        let _ = IndexFunctions::delete_existing_indexes(&collection, &mut indexes).await;
        let option: Option<CreateIndexesResult> = None;
        if indexes.is_empty() {
            return Ok(option);
        }
        Ok(Some(collection.create_indexes(indexes).await?))
    }
}