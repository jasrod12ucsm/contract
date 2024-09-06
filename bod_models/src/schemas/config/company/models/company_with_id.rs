use serde::{Deserialize, Serialize};
use bson::{oid::ObjectId, DateTime};

use crate::schemas::{config::company::company::{Sensible, SocialNetworks}, location::{country::models::short_country::ShortCountry, region::region::Region}, mst::user::models::short_user::ShortUser};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyWithId {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub sensible: Sensible,
    pub logo: String,
    #[serde(rename = "largeLogo")]
    pub large_logo: String,
    #[serde(rename = "smallLogo")]
    pub small_logo: String,
    pub emails: Vec<String>,
    pub name: String,
    #[serde(rename = "dispĺayName")]
    pub display_name: String,
    pub user: ShortUser,
    pub country: ShortCountry,
    pub region: Region,
    pub website: Option<String>,
    #[serde(rename = "employeeCount")]
    pub employee_count: String,
    pub vision: String,
    pub mission: String,
    pub categories: Option<ObjectId>,
    pub social: SocialNetworks,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "isActive")]
    pub is_active: bool,
}