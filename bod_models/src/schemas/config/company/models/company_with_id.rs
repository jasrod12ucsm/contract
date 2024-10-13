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
    pub sensible: Option<Sensible>,
    logo: Option<String>,
    #[serde(rename = "largeLogo")]
    large_logo: Option<String>,
    #[serde(rename = "smallLogo")]
    small_logo: Option<String>,
    emails: Vec<String>,
    name: Option<String>,
    #[serde(rename = "dispĺayName")]
    display_name: Option<String>,
    country: ShortCountry,
    region: ShortRegion,
    website: Option<String>,
    #[serde(rename = "employeeCount")]
    employee_count: i32,
    vision: Option<String>,
    mission: Option<String>,
    #[serde(rename = "quantityAddress")]
    pub quantity_restaurant: i32,
    #[serde(rename = "cardPlan")]
    pub card_plan: String,
    categories: Option<Vec<ObjectId>>,
    social: SocialNetworks,
    #[serde(rename = "createdAt")]
    created_at: DateTime,
    #[serde(rename = "updatedAt")]
    updated_at: DateTime,
    #[serde(rename = "isDeleted")]
    is_deleted: bool,
    #[serde(rename = "isActive")]
    is_active: bool,
}
