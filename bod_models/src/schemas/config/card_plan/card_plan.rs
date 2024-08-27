use async_trait::async_trait;
use bson::{doc, DateTime};
use mongodb::{options::IndexOptions, results::CreateIndexesResult, Client, IndexModel};
use serde::{Deserialize, Serialize};



use crate::shared::{index_functions::IndexFunctions, schema::{BaseColleccionNames, Schema}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceItems {
    pub item: String,
    pub included: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardPlan {
    pub order:i32,
    pub button: String,
    pub price: i32,
    pub shape: String,
    pub items: Vec<PriceItems>,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
}

impl CardPlan {
    pub fn new(button: String, price: i32, shape: String, items: Vec<PriceItems>,order:i32) -> CardPlan {
        CardPlan {
            order,
            button,
            price,
            shape,
            items,
            is_active: true,
            updated_at: DateTime::now(),
        }
    }
}

pub struct CardPlanSchema;

impl BaseColleccionNames for CardPlan {
    fn get_collection_name() -> &'static str {
        "cnf-cardplan"
    }

    fn get_database_name() -> &'static str {
        "bod"
    }
}

#[async_trait]
impl Schema for CardPlanSchema {
    fn get_collection_name(&self) -> &'static str {
        CardPlan::get_collection_name()
    }

    fn get_database_name(&self) -> &'static str {
        CardPlan::get_database_name()
    }

    async fn set_indexes(
        &self,
        client: &Client,
    ) -> Result<Option<CreateIndexesResult>, mongodb::error::Error> {
        let collection = client
            .database(self.get_database_name())
            .collection::<CardPlan>(self.get_collection_name());
        let mut indexes: Vec<IndexModel> = vec![];

        let is_active_index = IndexModel::builder()
            .keys(doc! {"isActive": 1})
            .options(
                IndexOptions::builder()
                    .name("isActive".to_string())
                    .build(),
            )
            .build();
        indexes.push(is_active_index);



        let updated_at_index = IndexModel::builder()
            .keys(doc! {"updatedAt": 1})
            .options(
                IndexOptions::builder()
                    .name("updatedAt".to_string())
                    .build(),
            )
            .build();
        indexes.push(updated_at_index);

        let _ = IndexFunctions::delete_existing_indexes(&collection, &mut indexes).await;
        let option: Option<CreateIndexesResult> = None;
        if indexes.len() == 0 {
            return Ok(option);
        }
        Ok(Some(collection.create_indexes(indexes).await?))
    }
}
