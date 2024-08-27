use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AccountingEntryWithId{
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(rename = "accountingAccountId")]
    pub accounting_account_id: ObjectId,
    pub description: String,
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