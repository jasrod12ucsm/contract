use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use crate::{schemas::{
    location::{
        country::models::short_country::ShortCountry, region::models::short_region::ShortRegion,
    },
    mst::{restaurant::restaurant::Restaurant, user::models::atention_hour::AtentionHour},
}, shared::geo_point::GeoPoint};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestaurantWithId {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub location: GeoPoint,
    #[serde(rename = "openHour")]
    pub open_hour: AtentionHour,
    #[serde(rename = "closeHour")]
    pub close_hour: AtentionHour,
    pub country: ShortCountry,
    pub region: ShortRegion,
    pub name: String,
    pub address: String,
    #[serde(rename = "contentTypeIds")]
    pub content_type_ids: Vec<ObjectId>,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "timeZone")]
    pub time_zone: String,
    #[serde(rename = "employeeCount")]
    pub employee_count: i32,
    #[serde(rename = "companyId")]
    pub company_id: ObjectId,
}

// Implementa Display para RestaurantWithId
impl std::fmt::Display for RestaurantWithId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RestaurantWithId: {}", self.id)
    }
}

// Implementa From para RestaurantWithId con id
impl RestaurantWithId {
    pub fn from_restaurant_and_id(restaurant: Restaurant, id: ObjectId) -> Self {
        Self {
            employee_count: restaurant.employee_count,
            content_type_ids: restaurant.content_type_ids,
            company_id: restaurant.company_id,
            id,
            location:restaurant.location,
            open_hour: restaurant.open_hour,
            close_hour: restaurant.close_hour,
            country: restaurant.country,
            region: restaurant.region,
            name: restaurant.name,
            address: restaurant.address,
            is_active: restaurant.is_active,
            is_deleted: restaurant.is_deleted,
            updated_at: restaurant.updated_at,
            created_at: restaurant.created_at,
            time_zone: restaurant.time_zone,
        }
    }
}
