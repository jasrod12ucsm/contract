
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::schemas::
    location::{country::models::short_country::ShortCountry, region::models::short_region::ShortRegion}
;

use super::models::{atention_hour::AtentionHour, identification::Identification};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAttributes {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub country: ShortCountry,
    pub region: ShortRegion,
    pub identification: Identification,
    pub phone: String,
    pub image: Option<String>,
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    #[serde(rename = "childsIds")]
    pub childs_ids: Option<Vec<ObjectId>>,
    pub address: String,
    pub lvl: i32,
    pub logo: Option<String>,
    #[serde(rename = "typeProvider")]
    pub type_provider: String,
    #[serde(rename = "employedBy")]
    pub employed_by: Option<ObjectId>,
    #[serde(rename = "closeHour")]
    pub close_hour: AtentionHour,
    #[serde(rename = "openHour")]
    pub open_hour: AtentionHour,
    pub birthdate: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
}

impl UserAttributes {
    pub fn new_client(
        user_config: ObjectId,
        identification: Identification,
        phone: String,
        address: String,
        country: ShortCountry,
        region: ShortRegion,
        birthdate: String,
        type_provider: String,
    ) -> UserAttributes {
        UserAttributes {
            id:user_config,
            birthdate,
            region,
            country,
            lvl: 0, //client lvl
            identification,
            phone,
            image: None,
            address,
            parent_id: None,
            childs_ids: None,
            logo: None,
            type_provider,
            employed_by: None,
            close_hour: AtentionHour::create_empty(),
            open_hour: AtentionHour::create_empty(),
            is_active: true,
            is_deleted: false,
        }
    }
}
