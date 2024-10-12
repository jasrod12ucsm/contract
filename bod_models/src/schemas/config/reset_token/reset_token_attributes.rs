
use bson::{doc, oid::ObjectId, DateTime};


use serde::{Deserialize, Serialize};

use super::reset_token::Device;

#[derive(Serialize, Deserialize,Debug)]
pub struct ResetTokenAttributes {
    pub devices: Vec<Device>,
    #[serde(rename = "userId")]
    pub user_id: ObjectId,
    #[serde(rename = "created")]
    pub created: DateTime,
    #[serde(rename = "authCode")]
    pub auth_code:i32,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
}


impl ResetTokenAttributes {
    pub fn new(token: String, user_id: ObjectId,auth_code:i32,os:String,mac:String) -> Self {
        Self {
            auth_code,
            devices: vec![
                Device{
                    os:os.to_string(),
                    mac:mac.to_string(),
                    token:token.to_string()
                }
            ],
            user_id,
            created: DateTime::now(),
            is_deleted: false,
        }
    }
}