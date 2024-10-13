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
pub struct Render {
    pub order: i32,
    pub button: String,
    pub price: i32,
    pub shape: String,
    pub items: Vec<PriceItems>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestaurantsData {
    #[serde(rename = "totalPrice")]
    pub total_price: i32,
    #[serde(rename = "planToken")]
    pub plan_token: String,
    #[serde(rename = "numRestaurants")]
    pub num_restaurants: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardPlan {
    #[serde(rename = "_id")]
    pub id:String,//name
    pub render: Render,
    #[serde(rename = "pricePerRestaurant")]
    pub price_per_restaurant: i32,
    #[serde(rename = "restaurantsData")]
    pub restaurants_data: Vec<RestaurantsData>,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
    #[serde(rename = "priceActualizedDate")]
    pub price_actualized_date: Option<DateTime>,
}

impl CardPlan {
    pub fn new(
        render: Render,
        price_per_restaurant: i32,
        restaurants_data: Vec<RestaurantsData>,
        id: String,
    ) -> CardPlan {
        CardPlan {
            price_actualized_date: None,
            render,
            price_per_restaurant,
            restaurants_data,
            is_active: true,
            updated_at: DateTime::now(),
            id
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