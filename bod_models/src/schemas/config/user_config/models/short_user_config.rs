use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use super::{user_config_with_id::UserConfigWithId, user_config_without_password::UserConfigWithoutPassword};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortUserConfig {
    #[serde(rename="_id")]
    pub id: ObjectId,
    pub names: String,
    pub surnames: String,
    pub email: String,
    #[serde(rename="isActive")]
    pub is_active: bool,
    #[serde(rename="isDelete")]
    pub is_delete: bool,
}


impl From<UserConfigWithId> for ShortUserConfig {
    fn from(value: UserConfigWithId) -> Self {
        ShortUserConfig {
            id:value.id,
            names: value.names,
            surnames: value.surnames,
            email: value.email,
            is_active: value.is_active,
            is_delete: value.is_delete,
        }
    }
}
impl From<UserConfigWithoutPassword> for ShortUserConfig {
    fn from(value: UserConfigWithoutPassword) -> Self {
        ShortUserConfig {
            id: value.id,
            names: value.names,
            surnames: value.surnames,
            email: value.email,
            is_active: value.is_active,
            is_delete: value.is_delete,
        }
    }
}
