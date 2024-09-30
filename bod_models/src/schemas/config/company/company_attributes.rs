use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::schemas::{
    location::{country::models::short_country::ShortCountry, region::models::short_region::ShortRegion},
    mst::user::models::short_user::ShortUser,
};

use super::company::{Sensible, SocialNetworks};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyAttributes {
    sensible:Sensible,
    logo: String,
    #[serde(rename = "largeLogo")]
    large_logo: String,
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
    quantity_restaurant: i32,
    #[serde(rename="cardPlan")]
    card_plan: ObjectId,
    categories: Option<Vec<ObjectId>>,
    social: SocialNetworks,
    #[serde(rename="isDeleted")]
    is_deleted: bool,
    #[serde(rename="isActive")]
    is_active: bool,
}

pub struct CompanyAttributesBuilder {
    sensible: Option<Sensible>,
    logo: Option<String>,
    large_logo: Option<String>,
    small_logo: Option<String>,
    emails: Option<Vec<String>>,
    name: Option<String>,
    display_name: Option<String>,
    user: Option<ShortUser>,
    country: Option<ShortCountry>,
    region: Option<ShortRegion>,
    website: Option<Option<String>>,
    quantity_restaurant: Option<i32>,
    employee_count: Option<String>,
    vision: Option<String>,
    mission: Option<String>,
    categories: Option<Option<Vec<ObjectId>>>,
    card_plan: Option<ObjectId>,
    social: Option<SocialNetworks>,
    is_deleted: Option<bool>,
    is_active: Option<bool>,
}

impl CompanyAttributesBuilder {
    pub fn new() -> Self {
        Self {
            card_plan: None,
            quantity_restaurant: None,
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

    pub fn with_region(mut self, region: ShortRegion) -> Self {
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

    pub fn with_categories(mut self, categories: Option<Vec<ObjectId>>) -> Self {
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
    pub fn with_quantity_restaurant(mut self, quantity_restaurant: i32) -> Self {
        self.quantity_restaurant = Some(quantity_restaurant);
        self
    }
    pub fn with_card_plan(mut self, card_plan: ObjectId) -> Self {
        self.card_plan = Some(card_plan);
        self
    }

    pub fn build(self) -> Result<CompanyAttributes, &'static str> {
        Ok(CompanyAttributes {
            sensible: self.sensible.ok_or("sensible is required")?,
            logo: self.logo.ok_or("logo is required")?,
            large_logo: self.large_logo.ok_or("large_logo is required")?,
            small_logo: self.small_logo.ok_or("small_logo is required")?,
            emails: self.emails.ok_or("emails are required")?,
            name: self.name.ok_or("name is required")?,
            display_name: self.display_name.ok_or("display_name is required")?,
            country: self.country.ok_or("country is required")?,
            region: self.region.ok_or("region is required")?,
            website: self.website.unwrap_or(None),
            employee_count: self.employee_count.ok_or("employee_count is required")?,
            vision: self.vision.ok_or("vision is required")?,
            mission: self.mission.ok_or("mission is required")?,
            categories: self.categories.unwrap_or(None),
            quantity_restaurant: self
                .quantity_restaurant
                .ok_or("quantity_restaurant is required")?,
            social: self.social.ok_or("social is required")?,
            card_plan: self.card_plan.ok_or("card_plan is required")?,
            is_deleted: self.is_deleted.ok_or("is_deleted is required")?,
            is_active: self.is_active.ok_or("is_active is required")?,
        })
    }
}
