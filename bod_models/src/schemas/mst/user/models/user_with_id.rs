use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use crate::schemas::{config::user_config::models::short_user_config::ShortUserConfig, location::{country::models::short_country::ShortCountry, region::region::Region}, mst::user::user::User};

use super::identification::Identification;






#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserWithId {
    #[serde(rename="_id")]
    pub id:ObjectId,
    pub frecuency: Option<Vec<String>>,
    #[serde(rename = "userConfig")]
    pub user_config: ShortUserConfig,
    pub country:ShortCountry,
    pub region:Region,
    pub identification: Identification,
    pub phone: String,
    pub lvl:i32,
    pub image: Option<String>,
    #[serde(rename = "parentId")]
    pub parent_id:Option<String>,
    #[serde(rename = "childsIds")]
    pub childs_ids:Option<i32>,
    pub address: String,
    #[serde(rename = "typeProvider")]
    pub type_provider: Option<String>,
    #[serde(rename = "employedBy")]
    pub employed_by: Option<i32>,
    #[serde(rename = "closeHour")]
    pub close_hour: Option<String>,
    #[serde(rename = "openHour")]
    pub open_hour: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
}
//implementa display para user with id
impl std::fmt::Display for UserWithId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UserWithId: {}", self.id)
    }
}

//implementa from para user with id con id
impl UserWithId{
    pub fn from_user_and_id(user:User,id:ObjectId)->Self{
        Self{
            region:user.region,
            country:user.country,
            lvl:user.lvl,
            id,
            frecuency:user.frecuency,
            user_config:user.user_config,
            identification:user.identification,
            phone:user.phone,
            image:user.image,
            parent_id:user.parent_id,
            childs_ids:user.childs_ids,
            address:user.address,
            type_provider:user.type_provider,
            employed_by:user.employed_by,
            close_hour:user.close_hour,
            open_hour:user.open_hour,
            created_at:user.created_at,
            updated_at:user.updated_at,
            is_active:user.is_active,
            is_deleted:user.is_deleted,
        }
    }
}