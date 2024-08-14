use async_trait::async_trait;
use bson::{doc, DateTime};
use mongodb::{options::IndexOptions, results::CreateIndexesResult, Client, IndexModel};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::shared::{index_functions::IndexFunctions, schema::{BaseColleccionNames, Schema}};

use super::models::region_with_id::RegionWithId;

#[derive(Serialize, Deserialize, Validate, Debug,Clone)]
pub struct Region {
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
impl From<RegionWithId> for Region {
    fn from(value: RegionWithId) -> Self {
        Self {
            code: value.code,
            full_name: value.full_name,
            country_id: value.country_id,
            is_active: value.is_active,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl Region {
    pub fn new(code: String, full_name: String, country_id: String) -> Self {
        let now = DateTime::now();
        Self {
            code,
            full_name,
            country_id,
            is_active: true, // Default to true, or change as needed
            created_at: now,
            updated_at: now,
        }
    }
}

pub struct RegionSchema;

impl BaseColleccionNames for Region {
    fn get_collection_name() -> &'static str {
        "loc-region"
    }

    fn get_database_name() -> &'static str {
        "bod"
    }
}


#[async_trait]
impl Schema for RegionSchema {
    fn get_collection_name(&self) -> &'static str {
        Region::get_collection_name()
    }

    fn get_database_name(&self) -> &'static str {
        Region::get_database_name()
    }

    async fn set_indexes(
        &self,
        client: &Client,
    ) -> Result<Option<CreateIndexesResult>, mongodb::error::Error> {
        let collection = client
            .database(self.get_database_name())
            .collection::<Region>(self.get_collection_name());
        let mut indexes: Vec<IndexModel> = vec![];
        
        // Create a unique index for the 'code' field
        let unique_code_index = IndexModel::builder()
            .keys(doc! {"code": 1})
            .options(
                IndexOptions::builder()
                    .unique(true)
                    .name("code".to_string())
                    .build(),
            )
            .build();
        indexes.push(unique_code_index);
        
        // Create a non-unique index for the 'countryId' field
        let country_id_index = IndexModel::builder()
            .keys(doc! {"countryId": 1})
            .options(
                IndexOptions::builder()
                    .name("countryId".to_string())
                    .build(),
            )
            .build();
        indexes.push(country_id_index);
        
        // Delete existing indexes and create new ones
        let _ = IndexFunctions::delete_existing_indexes(&collection, &mut indexes).await;
        let option: Option<CreateIndexesResult> = None;
        if indexes.len() == 0 {
            return Ok(option);
        }
        Ok(Some(collection.create_indexes(indexes).await?))
    }
}