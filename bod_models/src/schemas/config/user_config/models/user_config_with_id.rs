use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct UserConfigWithId {
    #[serde(rename="_id")]
    pub id:ObjectId,
    pub names: String,
    pub surnames: String,
    #[serde(rename="accountType")]
    pub account_type: String,
    pub email: String,
    #[serde(rename="isAuthenticated")]
    pub is_authenticated: bool,
    pub password: String,
    #[serde(rename="isActive")]
    pub is_active: bool,
    #[serde(rename="isDelete")]
    pub is_delete: bool,
    #[serde(rename="createdAt")]
    pub created_at: DateTime,
    #[serde(rename="updatedAt")]
    pub updated_at: DateTime,
}

