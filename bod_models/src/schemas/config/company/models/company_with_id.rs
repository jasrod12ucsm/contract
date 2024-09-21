use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use crate::schemas::{
    config::company::company::{Sensible, SocialNetworks},
    location::{country::models::short_country::ShortCountry, region::models::short_region::ShortRegion},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyWithId {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub sensible:Sensible,
    pub logo: String,
    #[serde(rename = "largeLogo")]
    pub large_logo: String,
    #[serde(rename = "smallLogo")]
    small_logo: String,
    emails: Vec<String>,
    name: String,
    #[serde(rename = "dispĺayName")]
    display_name: String,
    country: ShortCountry,
    region: ShortRegion,
    website: Option<String>,
    #[serde(rename="employeeCount")]
    employee_count: String,
    vision: String,
    mission: String,
    #[serde(rename="quantityRestaurant")]
    pub quantity_restaurant: i32,
    #[serde(rename="cardPlan")]
    pub card_plan: ObjectId,
    categories: Option<Vec<ObjectId>>,
    social: SocialNetworks,
    #[serde(rename="createdAt")]
    created_at: DateTime,
    #[serde(rename="updatedAt")]
    updated_at: DateTime,
    #[serde(rename="isDeleted")]
    is_deleted: bool,
    #[serde(rename="isActive")]
    is_active: bool,
}
