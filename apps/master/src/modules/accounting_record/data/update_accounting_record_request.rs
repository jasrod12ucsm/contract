use bod_models::schemas::mst::accounting_entry::models::short_accounting_entry::ShortAccountingEntry;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateAccountingRecordRequest {
    #[serde(rename = "accountingEntries")]
    pub accounting_entries: Option<Vec<ShortAccountingEntry>>,
    #[serde(rename = "transactionDocument")]
    pub transaction_document: Option<ObjectId>,
    pub company: Option<ObjectId>,
}