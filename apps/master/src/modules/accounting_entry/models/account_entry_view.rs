use bson::DateTime;
use serde::Serialize;

#[derive(Serialize)]
pub struct AccountingEntryView {
    #[serde(rename = "accountNumber")]
    pub account_number: String,

    pub description: String,
    pub debit: f64,
    pub credit: f64,
    #[serde(rename="isActive")]
    pub is_active: bool,
    #[serde(rename="isDeleted")]
    pub is_deleted: bool,
    #[serde(rename="createdAt")]
    pub created_at: DateTime,
    #[serde(rename="updatedAt")]
    pub updated_at: DateTime,
}