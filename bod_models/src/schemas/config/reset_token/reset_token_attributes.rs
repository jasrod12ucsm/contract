
use bson::{doc, oid::ObjectId, DateTime};


use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Debug)]
pub struct ResetTokenAttributes {
    pub token: String,
    #[serde(rename = "userId")]
    pub user_id: ObjectId,
    #[serde(rename = "userConfigId")]
    pub user_config_id: ObjectId,
    #[serde(rename = "created")]
    pub created: DateTime,
    #[serde(rename = "authCode")]
    pub auth_code:i32,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
}


impl ResetTokenAttributes {
    pub fn new(token: String, user_id: ObjectId,auth_code:i32,user_config_id:ObjectId) -> Self {
        Self {
            user_config_id,
            auth_code,
            token,
            user_id,
            created: DateTime::now(),
            is_deleted: false,
        }
    }
}