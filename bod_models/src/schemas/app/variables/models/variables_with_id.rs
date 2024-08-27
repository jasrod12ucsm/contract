use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AppVariablesWithId {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(rename = "whatsappLink")]
    pub whatsapp_link: String,
    #[serde(rename = "instagramLink")]
    pub instagram_link: String,
    #[serde(rename = "facebookLink")]
    pub facebook_link: String,
    pub phone:String,
    #[serde(rename="appName")]
    pub app_name: String,
}