use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResetTokenWithId {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub token: String,
    #[serde(rename = "userId")]
    pub user_id: ObjectId,
    #[serde(rename = "userConfigId")]
    pub user_config_id:ObjectId,
    #[serde(rename = "authCode")]
    pub auth_code: i32,
    pub created: DateTime,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
}
