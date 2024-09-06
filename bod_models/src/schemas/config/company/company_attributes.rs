use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::schemas::{
    location::{country::models::short_country::ShortCountry, region::region::Region},
    mst::user::models::short_user::ShortUser,
};

use super::company::{Sensible, SocialNetworks};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyAttributes {
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
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "isActive")]
    pub is_active: bool,
}

pub struct CompanyAttributesBuilder {
    id: Option<ObjectId>,
    sensible: Option<Sensible>,
    logo: Option<String>,
    large_logo: Option<String>,
    small_logo: Option<String>,
    emails: Option<Vec<String>>,
    name: Option<String>,
    display_name: Option<String>,
    user: Option<ShortUser>,
    country: Option<ShortCountry>,
    region: Option<Region>,
    website: Option<Option<String>>,
    employee_count: Option<String>,
    vision: Option<String>,
    mission: Option<String>,
    categories: Option<Option<ObjectId>>,
    social: Option<SocialNetworks>,
    is_deleted: Option<bool>,
    is_active: Option<bool>,
}

impl CompanyAttributesBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            sensible: None,
            logo: None,
            large_logo: None,
            small_logo: None,
            emails: None,
            name: None,
            display_name: None,
            user: None,
            country: None,
            region: None,
            website: None,
            employee_count: None,
            vision: None,
            mission: None,
            categories: None,
            social: None,
            is_deleted: None,
            is_active: None,
        }
    }

    pub fn with_id(mut self, id: ObjectId) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_sensible(mut self, sensible: Sensible) -> Self {
        self.sensible = Some(sensible);
        self
    }

    pub fn with_logo(mut self, logo: String) -> Self {
        self.logo = Some(logo);
        self
    }

    pub fn with_large_logo(mut self, large_logo: String) -> Self {
        self.large_logo = Some(large_logo);
        self
    }

    pub fn with_small_logo(mut self, small_logo: String) -> Self {
        self.small_logo = Some(small_logo);
        self
    }

    pub fn with_emails(mut self, emails: Vec<String>) -> Self {
        self.emails = Some(emails);
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_display_name(mut self, display_name: String) -> Self {
        self.display_name = Some(display_name);
        self
    }

    pub fn with_user(mut self, user: ShortUser) -> Self {
        self.user = Some(user);
        self
    }

    pub fn with_country(mut self, country: ShortCountry) -> Self {
        self.country = Some(country);
        self
    }

    pub fn with_region(mut self, region: Region) -> Self {
        self.region = Some(region);
        self
    }

    pub fn with_website(mut self, website: Option<String>) -> Self {
        self.website = Some(website);
        self
    }

    pub fn with_employee_count(mut self, employee_count: String) -> Self {
        self.employee_count = Some(employee_count);
        self
    }

    pub fn with_vision(mut self, vision: String) -> Self {
        self.vision = Some(vision);
        self
    }

    pub fn with_mission(mut self, mission: String) -> Self {
        self.mission = Some(mission);
        self
    }

    pub fn with_categories(mut self, categories: Option<ObjectId>) -> Self {
        self.categories = Some(categories);
        self
    }

    pub fn with_social(mut self, social: SocialNetworks) -> Self {
        self.social = Some(social);
        self
    }

    pub fn with_is_deleted(mut self, is_deleted: bool) -> Self {
        self.is_deleted = Some(is_deleted);
        self
    }

    pub fn with_is_active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }

    pub fn build(self) -> Result<CompanyAttributes, &'static str> {
        Ok(CompanyAttributes {
            id: self.id.ok_or("id is required")?,
            sensible: self.sensible.ok_or("sensible is required")?,
            logo: self.logo.ok_or("logo is required")?,
            large_logo: self.large_logo.ok_or("large_logo is required")?,
            small_logo: self.small_logo.ok_or("small_logo is required")?,
            emails: self.emails.ok_or("emails are required")?,
            name: self.name.ok_or("name is required")?,
            display_name: self.display_name.ok_or("display_name is required")?,
            user: self.user.ok_or("user is required")?,
            country: self.country.ok_or("country is required")?,
            region: self.region.ok_or("region is required")?,
            website: self.website.unwrap_or(None),
            employee_count: self.employee_count.ok_or("employee_count is required")?,
            vision: self.vision.ok_or("vision is required")?,
            mission: self.mission.ok_or("mission is required")?,
            categories: self.categories.unwrap_or(None),
            social: self.social.ok_or("social is required")?,
            is_deleted: self.is_deleted.ok_or("is_deleted is required")?,
            is_active: self.is_active.ok_or("is_active is required")?,
        })
    }
}