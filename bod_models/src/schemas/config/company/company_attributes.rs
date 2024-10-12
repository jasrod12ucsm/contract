use bson::{doc, oid::ObjectId, DateTime};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{schemas::location::{country::models::short_country::ShortCountry, region::models::short_region::ShortRegion}, shared::bson::to_bson::ToBson};

use super::company::{Sensible, SocialNetworks};

#[derive(Serialize, Deserialize, Clone, Builder)]
#[builder(setter(into), build_fn(validate = "Self::validate"))]
pub struct CompanyAttributes {
    #[serde(rename = "_id")]
    id: ObjectId,
    sensible: Sensible,
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
    employee_count: i32,
    vision: String,
    mission: String,
    #[serde(rename="cardPlan")]
    card_plan: String,
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

impl CompanyAttributesBuilder {
    fn validate(&self) -> Result<(), String> {
        if self.id.is_none() {
            return Err("Id is required".into());
        }
        if self.country.is_none() {
            return Err("Country is required".into());
        }
        if self.region.is_none() {
            return Err("Region is required".into());
        }
        if self.employee_count.is_none() {
            return Err("Employee count is required".into());
        }
        if self.card_plan.is_none() {
            return Err("Card plan is required".into());
        }
        if self.is_deleted.is_none() {
            return Err("Is deleted is required".into());
        }
        if self.is_active.is_none() {
            return Err("Is active is required".into());
        }
        Ok(())
    }
}


impl ToBson for CompanyAttributes {
    fn to_bson(&self) -> Result<bson::Document, bson::ser::Error> {
        let doc = bson::to_bson(self);
        if (&doc).is_err() {
            return Err(doc.err().unwrap().to_owned());
        }
        let document = doc! {
            "$set":doc.unwrap()
        };
        Ok(document)
    }
}
