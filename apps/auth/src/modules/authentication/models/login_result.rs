use bod_models::schemas::{
    config::user_config::models::short_user_config::ShortUserConfig,
    location::{
        country::models::short_country::ShortCountry, region::models::short_region::ShortRegion,
    },
    mst::{
        restaurant::models::restaurant_with_id::RestaurantWithId,
        user::models::{
            atention_hour::AtentionHour, identification::Identification, user_with_id::UserWithId,
        },
    },
};
use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResult {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(rename = "userConfig")]
    pub user_config: ShortUserConfig,
    pub country: ShortCountry,
    pub region: ShortRegion,
    pub identification: Identification,
    pub phone: String,
    pub lvl: i32,
    pub image: Option<String>,
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    #[serde(rename = "childsIds")]
    pub childs_ids: Option<Vec<ObjectId>>,
    pub address: String,
    #[serde(rename = "typeProvider")]
    pub type_provider: String,
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
    pub restaurant: Vec<LoginResultRestaurant>,
}
pub enum LoginResultRestaurantEnum {
    Restaurant(RestaurantWithId),
    ShortRestaurant(RestaurantWithId),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResultRestaurant {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub open_hour: Option<AtentionHour>,
    pub close_hour: Option<AtentionHour>,
    pub efective_area: Option<f64>,
    pub country: Option<ShortCountry>,
    pub region: Option<ShortRegion>,
    pub name: String,
    pub address: Option<String>,
    pub num_mesas: Option<i32>,
    pub is_active: Option<bool>,
    pub is_deleted: Option<bool>,
    pub updated_at: Option<DateTime>,
    pub created_at: Option<DateTime>,
    pub time_zone: Option<String>,
}

impl From<LoginResultRestaurantEnum> for LoginResultRestaurant {
    fn from(restaurant: LoginResultRestaurantEnum) -> Self {
        match restaurant {
            LoginResultRestaurantEnum::Restaurant(restaurant) => Self {
                id: restaurant.id,
                longitude: Some(restaurant.longitude),
                latitude: Some(restaurant.latitude),
                open_hour: Some(restaurant.open_hour),
                close_hour: Some(restaurant.close_hour),
                efective_area: Some(restaurant.efective_area),
                country: Some(restaurant.country),
                region: Some(restaurant.region),
                name: restaurant.name,
                address: Some(restaurant.address),
                num_mesas: Some(restaurant.num_mesas),
                is_active: Some(restaurant.is_active),
                is_deleted: Some(restaurant.is_deleted),
                updated_at: Some(restaurant.updated_at),
                created_at: Some(restaurant.created_at),
                time_zone: Some(restaurant.time_zone),
            },
            LoginResultRestaurantEnum::ShortRestaurant(restaurant) => Self {
                id: restaurant.id,
                longitude: None,
                latitude: None,
                open_hour: None,
                close_hour: None,
                efective_area: None,
                country: None,
                region: None,
                name: restaurant.name,
                address: None,
                num_mesas: None,
                is_active: None,
                is_deleted: None,
                updated_at: None,
                created_at: None,
                time_zone: None,
            },
        }
    }
}

impl LoginResult {
    pub fn from(user: &UserWithId, user_config: ShortUserConfig) -> Self {
        let user=user.to_owned();
        Self {
            id: user.id,
            user_config,
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
            restaurant: vec![],
        }
    }
    pub fn add_restaurant(&mut self, restaurant: LoginResultRestaurant) {
        self.restaurant.push(restaurant);
    }
}
