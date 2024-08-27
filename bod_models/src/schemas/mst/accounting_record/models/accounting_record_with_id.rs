use bson::{doc, oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use crate::schemas::mst::accounting_entry::models::accounting_entry_with_id::AccountingEntryWithId;
#[derive(Serialize, Deserialize)]
pub struct AccountingRecordWithId {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(rename = "accountingEntries")]
    pub accounting_entries: Vec<AccountingEntryWithId>,
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
