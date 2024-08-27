use bson::{doc, oid::ObjectId, DateTime};
use mongodb::{options::IndexOptions, results::CreateIndexesResult, Client, IndexModel};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    schemas::mst::accounting_entry::models::short_accounting_entry::ShortAccountingEntry,
    shared::{
        index_functions::IndexFunctions,
        schema::{BaseColleccionNames, Schema},
    },
};
#[derive(Serialize, Deserialize,Validate)]
pub struct AccountingRecord {
    #[serde(rename = "accountingEntries")]
    pub accounting_entries: Vec<ShortAccountingEntry,>,
    #[serde(rename = "transactionDocument")]
    pub transaction_document: Option<ObjectId>,
    pub company: ObjectId,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
}
impl AccountingRecord {
    pub fn new(
        accounting_entries: Vec<ShortAccountingEntry,>,
        transaction_document: Option<ObjectId>,
        company:ObjectId
    ) -> AccountingRecord {
        AccountingRecord {
            company,
            transaction_document,
            accounting_entries,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
            is_active: true,
            is_deleted: false,
        }
    }
}
pub struct AccountingRecordSchema;

impl BaseColleccionNames for AccountingRecord {
    fn get_collection_name() -> &'static str {
        "mst-accounting-record"
    }

    fn get_database_name() -> &'static str {
        "bod"
    }
}
#[async_trait::async_trait]
impl Schema for AccountingRecordSchema {
    fn get_collection_name(&self) -> &'static str {
        "mst-accounting-record"
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
            .collection::<AccountingRecord>(self.get_collection_name());

        let mut indexes: Vec<IndexModel> = vec![];

        // Index on accountingEntries
        let accounting_entries_index = IndexModel::builder()
            .keys(doc! {"accountingEntries": 1,"isActive":1,"isDeleted":1})
            .options(
                IndexOptions::builder()
                    .name("accountingEntries".to_string())
                    .build(),
            )
            .build();
        indexes.push(accounting_entries_index);
        let active_and_delete_index = IndexModel::builder()
            .keys(doc! {"isActive":1,"isDeleted":1})
            .options(IndexOptions::builder().name("isActive".to_string()).build())
            .build();
        indexes.push(active_and_delete_index);

        let _ = IndexFunctions::delete_existing_indexes(&collection, &mut indexes).await;
        let option: Option<CreateIndexesResult> = None;
        if indexes.len() == 0 {
            return Ok(option);
        }
        Ok(Some(collection.create_indexes(indexes).await?))
    }
}
