use async_trait::async_trait;
use bson::{doc, oid::ObjectId, DateTime, Decimal128};
use mongodb::{options::IndexOptions, results::CreateIndexesResult, Client, IndexModel};
use serde::{Deserialize, Serialize};

use crate::shared::{
    index_functions::IndexFunctions,
    schema::{BaseColleccionNames, Schema},
};

use super::models::comission::Comision;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionPayment {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "igv")]
    pub igv: Decimal128,
    #[serde(rename = "priceRequested")]
    pub price_requested: Decimal128, // Convertido a camelCase
    #[serde(rename = "price")]
    pub price: Decimal128,
    #[serde(rename = "comision")]
    pub comision: Comision,
    #[serde(rename = "targetToken")]
    pub target_token: String, // Convertido a camelCase
    #[serde(rename = "companyId")]
    pub company_id: ObjectId, // Convertido a camelCase
    #[serde(rename = "cardPlanId")]
    pub card_plan_id: ObjectId, // Convertido a camelCase
    #[serde(rename = "createdAt")]
    pub created_at: DateTime, // Convertido a camelCase
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime, // Convertido a camelCase
    #[serde(rename = "isActive")]
    pub is_active: bool, // Convertido a camelCase
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool, // Convertido a camelCase
    #[serde(rename = "error")]
    pub error: String,
}

impl SubscriptionPayment {
    pub fn new(
        id: String,
        igv: Decimal128,
        price_requested: Decimal128,
        price: Decimal128,
        comision: Comision,
        target_token: String,
        company_id: ObjectId,
        card_plan_id: ObjectId,
        error: String,
    ) -> Self {
        let now = DateTime::now();
        Self {
            id,
            igv,
            price_requested,
            price,
            comision,
            target_token,
            company_id,
            card_plan_id,
            created_at: now,
            updated_at: now,
            is_active: true,
            is_deleted: false,
            error,
        }
    }
}

pub struct SubscriptionPaymentSchema;

impl BaseColleccionNames for SubscriptionPayment {
    fn get_collection_name() -> &'static str {
        "sub-payment"
    }

    fn get_database_name() -> &'static str {
        "bod"
    }
}

#[async_trait]
impl Schema for SubscriptionPaymentSchema {
    fn get_collection_name(&self) -> &'static str {
        SubscriptionPayment::get_collection_name()
    }

    fn get_database_name(&self) -> &'static str {
        SubscriptionPayment::get_database_name()
    }

    async fn set_indexes(
        &self,
        client: &Client,
    ) -> Result<Option<CreateIndexesResult>, mongodb::error::Error> {
        let collection = client
            .database(self.get_database_name())
            .collection::<SubscriptionPayment>(self.get_collection_name());
        let mut indexes: Vec<IndexModel> = vec![];
        let unique_index = IndexModel::builder()
            .keys(doc! {"_id":1, "isDeleted":1, "isActive":1})
            .options(
                IndexOptions::builder()
                    .unique(true)
                    .name("_id".to_string())
                    .build(),
            )
            .build();
        let company_id_index = IndexModel::builder()
            .keys(doc! {"companyId": 1,"isDeleted":1,"isActive":1})
            .options(
                IndexOptions::builder()
                    .name("companyId_index".to_string())
                    .build(),
            )
            .build();
        indexes.push(company_id_index);
        indexes.push(unique_index);
        let _ = IndexFunctions::delete_existing_indexes(&collection, &mut indexes).await;
        let option: Option<CreateIndexesResult> = None;
        if indexes.len() == 0 {
            return Ok(option);
        }
        Ok(Some(collection.create_indexes(indexes).await?))
    }
}
