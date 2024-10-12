use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use crate::schemas::config::reset_token::reset_token::Device;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResetTokenWithId {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub devices: Vec<Device>,
    #[serde(rename = "userId")]
    pub user_id: ObjectId,
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
