use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use crate::{
    schemas::{
        location::{
            country::models::short_country::ShortCountry, region::models::short_region::ShortRegion,
        },
        mst::user::models::atention_hour::AtentionHour,
    },
    shared::geo_point::GeoPoint,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct WareHouse {
    pub location: GeoPoint,
    #[serde(rename = "openHour")]
    pub open_hour: AtentionHour,
    #[serde(rename = "closeHour")]
    pub close_hour: AtentionHour,
    pub country: ShortCountry,
    pub region: ShortRegion,
    pub name: String,
    pub address: String,
    #[serde(rename = "employeeCount")]
    pub employee_count: i32,
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
    #[serde(rename = "companyId")]
    pub company_id: ObjectId,
}
