

use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct UserConfigAttributes{
    pub names: String,
    pub surnames: String,
    #[serde(rename = "accountType")]
    pub account_type: String,
    pub email: String,
    #[serde(rename = "isAuthenticated")]
    pub is_authenticated: bool,
    pub password: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isDelete")]
    pub is_delete: bool,
}

impl UserConfigAttributes {
    pub fn new_client(
        names: String,
        surnames: String,
        email: String,
        password: String,
    ) -> Self {
        Self {
            names,
            surnames,
            account_type:"P".to_string(),
            email,
            password,
            is_authenticated: false,
            is_active: true,
            is_delete: false,
        }
    }
}
