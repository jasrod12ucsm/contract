use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};


use super::user_config_with_id::UserConfigWithId;
#[derive(Deserialize,Serialize,Debug)]
pub struct UserConfigWithoutPassword {
    pub id: ObjectId,
    pub names: String,
    pub surnames: String,
    #[serde(rename="accountType")]
    pub account_type: String,
    #[serde(rename="isAuthenticated")]
    pub is_authenticated:bool,
    pub email: String,
    #[serde(rename="isActive")]
    pub is_active: bool,
    #[serde(rename="isDelete")]
    pub is_delete: bool,
}
impl From<UserConfigWithId> for UserConfigWithoutPassword {
    fn from(value: UserConfigWithId) -> Self {
        UserConfigWithoutPassword {
            id: value.id,
            names: value.names,
            surnames: value.surnames,
            account_type: value.account_type,
            email: value.email,
            is_authenticated: value.is_authenticated,
            is_active: value.is_active,
            is_delete: value.is_delete,
        }
    }
}
