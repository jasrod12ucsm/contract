
use serde::{Deserialize, Serialize};

use crate::schemas::{
    config::user_config::models::short_user_config::ShortUserConfig,
    location::{country::models::short_country::ShortCountry, region::region::Region},
};

use super::models::identification::Identification;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAttributes {
    pub frecuency: Option<Vec<String>>,
    pub country: ShortCountry,
    pub region: Region,
    #[serde(rename = "userConfig")]
    pub user_config: ShortUserConfig,
    pub identification: Identification,
    pub phone: String,
    pub image: Option<String>,
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    #[serde(rename = "childsIds")]
    pub childs_ids: Option<i32>,
    pub address: String,
    pub lvl: i32,
    pub logo: Option<String>,
    #[serde(rename = "typeProvider")]
    pub type_provider: Option<String>,
    #[serde(rename = "employedBy")]
    pub employed_by: Option<i32>,
    #[serde(rename = "closeHour")]
    pub close_hour: Option<String>,
    #[serde(rename = "openHour")]
    pub open_hour: Option<String>,
    pub birthdate: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
}

impl UserAttributes {
    pub fn new_client(
        user_config: ShortUserConfig,
        identification: Identification,
        phone: String,
        address: String,
        country: ShortCountry,
        region: Region,
        birthdate: String,
    ) -> UserAttributes {
        UserAttributes {
            birthdate,
            region,
            country,
            lvl: 0, //client lvl
            frecuency: None,
            user_config,
            identification,
            phone,
            image: None,
            address,
            parent_id: None,
            childs_ids: None,
            logo: None,
            type_provider: None,
            employed_by: None,
            close_hour: None,
            open_hour: None,
            is_active: true,
            is_deleted: false,
        }
    }
}
