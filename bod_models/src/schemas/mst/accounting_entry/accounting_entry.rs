use bson::{doc, oid::ObjectId, DateTime};
use mongodb::{options::IndexOptions, results::CreateIndexesResult, Client, IndexModel};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::shared::{index_functions::IndexFunctions, schema::{BaseColleccionNames, Schema}};

#[derive(Serialize, Deserialize,Validate)]
pub struct AccountingEntry {
    #[serde(rename = "accountingAccountId")]
    pub accounting_account_id: ObjectId,
    pub description: String,
    pub company:Option<ObjectId>,
    #[serde(rename = "entryType")]
    pub entry_type: String,
    pub credit: f64,
    pub debit: f64,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
}

pub struct AccountingEntrySchema;

impl BaseColleccionNames for AccountingEntry {
    fn get_collection_name() -> &'static str {
        "mst-accounting-entry"
    }

    fn get_database_name() -> &'static str {
        "bod"
    }
}

#[async_trait::async_trait]
impl Schema for AccountingEntrySchema {
    fn get_collection_name(&self) -> &'static str {
        "mst-accounting-entry"
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
            .collection::<AccountingEntry>(self.get_collection_name());

        let mut indexes: Vec<IndexModel> = vec![];

        // Índice en accountingAccountId
        let accounting_account_id_index = IndexModel::builder()
            .keys(doc! {"accountingAccountId": 1})
            .options(
                IndexOptions::builder()
                    .name("accountingAccountId".to_string())
                    .build(),
            )
            .build();
        indexes.push(accounting_account_id_index);

        // Índice en createdAt
        let created_at_index = IndexModel::builder()
            .keys(doc! {"createdAt": 1})
            .options(
                IndexOptions::builder()
                    .name("createdAt".to_string())
                    .build(),
            )
            .build();
        indexes.push(created_at_index);

        // Índice en updatedAt
        let updated_at_index = IndexModel::builder()
            .keys(doc! {"updatedAt": 1})
            .options(
                IndexOptions::builder()
                    .name("updatedAt".to_string())
                    .build(),
            )
            .build();
        indexes.push(updated_at_index);

        // Índice en isActive
        let is_active_index = IndexModel::builder()
            .keys(doc! {"isActive": 1})
            .options(
                IndexOptions::builder()
                    .name("isActive".to_string())
                    .build(),
            )
            .build();
        indexes.push(is_active_index);

        // Índice en isDeleted
        let is_deleted_index = IndexModel::builder()
            .keys(doc! {"isDeleted": 1})
            .options(
                IndexOptions::builder()
                    .name("isDeleted".to_string())
                    .build(),
            )
            .build();
        indexes.push(is_deleted_index);

        // Índice compuesto en createdAt e isActive
        let compound_index = IndexModel::builder()
            .keys(doc! {"createdAt": 1, "isActive": 1})
            .options(
                IndexOptions::builder()
                    .name("createdAt_isActive".to_string())
                    .build(),
            )
            .build();
        indexes.push(compound_index);

        let _ = IndexFunctions::delete_existing_indexes(&collection, &mut indexes).await;
        let option: Option<CreateIndexesResult> = None;
        if indexes.len() == 0 {
            return Ok(option);
        }
        Ok(Some(collection.create_indexes(indexes).await?))
    }
}