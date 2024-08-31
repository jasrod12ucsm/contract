use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};



use super::{identification::Identification, user_with_id::UserWithId};






#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortUser {
    #[serde(rename="_id")]
    pub id: ObjectId,
    #[serde(rename = "userConfigId")]
    pub user_config: ObjectId,
    pub identification: Identification,
    pub phone: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
}



impl From<UserWithId> for ShortUser {
    fn from(value: UserWithId) -> Self {
        Self {
            id: value.id,
            user_config: value.user_config,
            identification: value.identification,
            phone: value.phone,
            is_active: value.is_active,
            is_deleted: value.is_deleted,
        }
    }
}