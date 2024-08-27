use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug)]
pub struct AccountingAccountWithId{
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(rename = "accountNumber")]
    pub account_number: String,
    #[serde(rename = "accountName")]
    pub account_name: String,
    pub company: Option<ObjectId>,
    #[serde(rename = "accountType")]
    pub account_type: String,
    #[serde(rename = "accountCategory")]
    pub account_category: String,
    #[serde(rename = "parentAccount")]
    pub parent_account: Option<ObjectId>,
    #[serde(rename = "accountBalance")]
    pub balance: f64,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
}