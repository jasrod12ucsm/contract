use bod_models::schemas::{
    config::user_config::models::short_user_config::ShortUserConfig,
    location::{
        country::models::short_country::ShortCountry, region::models::short_region::ShortRegion,
    },
    mst::user::models::{
        atention_hour::AtentionHour, identification::Identification, user_with_id::UserWithId,
    },
};
use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResult {
    pub user: LoginResutlUser,
    pub token: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResutlUser {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub country: ShortCountry,
    pub region: ShortRegion,
    #[serde(rename = "userConfig")]
    pub user_config: ShortUserConfig,
    pub identification: Identification,
    pub phone: String,
    pub image: Option<String>,
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    #[serde(rename = "childsIds")]
    pub childs_ids: Option<Vec<ObjectId>>,
    pub address: String,
    pub lvl: i32,
    #[serde(rename = "typeProvider")]
    pub type_provider: String, //COMPANY, ATM, chef waiter
    #[serde(rename = "employedBy")]
    pub employed_by: Option<ObjectId>,
    #[serde(rename = "closeHour")]
    pub close_hour: AtentionHour,
    #[serde(rename = "openHour")]
    pub open_hour: AtentionHour,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
    pub birthdate: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
}

impl LoginResult {
    pub fn from(user: &UserWithId, user_config: ShortUserConfig,token:String,renew_token:String) -> Self {
        let user = user.to_owned();
        Self {
            refresh_token: renew_token,
            user: LoginResutlUser {
                id: user.id,
                user_config: user_config,
                country: user.country,
                region: user.region,
                identification: user.identification,
                phone: user.phone,
                lvl: user.lvl,
                image: user.image,
                parent_id: user.parent_id,
                childs_ids: user.childs_ids,
                address: user.address,
                type_provider: user.type_provider,
                employed_by: user.employed_by,
                close_hour: user.close_hour,
                open_hour: user.open_hour,
                created_at: user.created_at,
                updated_at: user.updated_at,
                birthdate: user.birthdate,
                is_active: user.is_active,
                is_deleted: user.is_deleted,
            },
            token
        }
    }
}
