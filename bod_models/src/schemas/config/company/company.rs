use async_trait::async_trait;
use bson::{doc, oid::ObjectId, DateTime};
use derive_builder::Builder;
use mongodb::{options::IndexOptions, results::CreateIndexesResult, Client, IndexModel};
use serde::{Deserialize, Serialize};

use crate::{
    schemas::{
        config::reset_token::reset_token::ResetToken,
        location::{
            country::models::short_country::ShortCountry, region::models::short_region::ShortRegion,
        },
    },
    shared::{
        index_functions::IndexFunctions,
        schema::{BaseColleccionNames, Schema},
    },
};

#[derive(Serialize, Deserialize, Clone, Builder)]
#[builder(setter(into), build_fn(validate = "Self::validate"))]
pub struct Company {
    #[serde(rename = "_id")]
    id: ObjectId,
    sensible: Option<Sensible>,
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
    quantity_restaurant: i32,
    #[serde(rename = "cardPlan")]
    card_plan: String,
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

#[derive(Serialize, Deserialize, Debug, Clone,Default)]
pub struct Sensible {
    #[serde(rename = "creditCards")]
    pub credit_cards: Vec<SensibleCard>, //la que esta en la posicion 0 es la principal la que esta rindiendo actualmente
    pub subscription: String,
    #[serde(rename = "clientToken")]
    pub client_token: String,
}



#[derive(Serialize, Deserialize, Debug, Clone,Default)]
pub struct SensibleCard {
    pub token: String,
    #[serde(rename = "lastFourDigits")]
    pub last_four_digits: i32,
    #[serde(rename = "isUsedCard")]
    pub is_used_card: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone,Default)]
pub struct SocialNetworks {
    #[serde(rename = "whatsAppNumbers")]
    whats_app_numbers: Option<Vec<String>>,
    discord: Option<String>,
    instagram: Option<String>,
    #[serde(rename = "contactNumbers")]
    contact_numbers: Option<String>,
    linkedin: Option<String>,
    facebook: Option<String>,
    snapchat: Option<String>,
    tiktok: Option<String>,
    youtube: Option<String>,
    pinterest: Option<String>,
    telegram: Option<String>,
    wechat: Option<String>,
    reddit: Option<String>,
}
impl CompanyBuilder {
    fn validate(&self) -> Result<(), String> {
        if self.sensible.is_none() {
            return Err("Sensible is required".into());
        }
        if self.logo.is_none() {
            return Err("Logo is required".into());
        }
        if self.large_logo.is_none() {
            return Err("Large logo is required".into());
        }
        if self.small_logo.is_none() {
            return Err("Small logo is required".into());
        }
        if self.emails.is_none() {
            return Err("Emails are required".into());
        }
        if self.name.is_none() {
            return Err("Name is required".into());
        }
        if self.display_name.is_none() {
            return Err("Display name is required".into());
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
        if self.vision.is_none() {
            return Err("Vision is required".into());
        }
        if self.mission.is_none() {
            return Err("Mission is required".into());
        }
        if self.quantity_restaurant.is_none() {
            return Err("Quantity restaurant is required".into());
        }
        if self.card_plan.is_none() {
            return Err("Card plan is required".into());
        }
        if self.social.is_none() {
            return Err("Social is required".into());
        }
        if self.is_deleted.is_none() {
            return Err("Is deleted is required".into());
        }
        if self.is_active.is_none() {
            return Err("Is active is required".into());
        }
        Ok(())
    }

    pub fn build_partial_update(&self) -> bson::Document {
        let mut doc = doc! {};
        if let Some(sensible) = &self.sensible {
            doc.insert("sensible", bson::to_bson(sensible).unwrap());
        }
        if let Some(logo) = &self.logo {
            doc.insert("logo", logo);
        }
        if let Some(large_logo) = &self.large_logo {
            doc.insert("largeLogo", large_logo);
        }
        if let Some(small_logo) = &self.small_logo {
            doc.insert("smallLogo", small_logo);
        }
        if let Some(emails) = &self.emails {
            doc.insert("emails", emails);
        }
        if let Some(name) = &self.name {
            doc.insert("name", name);
        }
        if let Some(display_name) = &self.display_name {
            doc.insert("displayName", display_name);
        }
        if let Some(country) = &self.country {
            doc.insert("country", bson::to_bson(country).unwrap());
        }
        if let Some(region) = &self.region {
            doc.insert("region", bson::to_bson(region).unwrap());
        }
        if let Some(website) = &self.website {
            doc.insert("website", website);
        }
        if let Some(employee_count) = &self.employee_count {
            doc.insert("employeeCount", employee_count);
        }
        if let Some(vision) = &self.vision {
            doc.insert("vision", vision);
        }
        if let Some(mission) = &self.mission {
            doc.insert("mission", mission);
        }
        if let Some(quantity_restaurant) = &self.quantity_restaurant {
            doc.insert("quantityRestaurant", quantity_restaurant);
        }
        if let Some(card_plan) = &self.card_plan {
            doc.insert("cardPlan", card_plan);
        }
        if let Some(categories) = &self.categories {
            doc.insert("categories", categories);
        }
        if let Some(social) = &self.social {
            doc.insert("social", bson::to_bson(social).unwrap());
        }
        if let Some(created_at) = &self.created_at {
            doc.insert("createdAt", created_at);
        }
        if let Some(updated_at) = &self.updated_at {
            doc.insert("updatedAt", updated_at);
        }
        if let Some(is_deleted) = &self.is_deleted {
            doc.insert("isDeleted", is_deleted);
        }
        if let Some(is_active) = &self.is_active {
            doc.insert("isActive", is_active);
        }
        doc
    }
}

pub struct CompanySchema;

impl BaseColleccionNames for Company {
    fn get_collection_name() -> &'static str {
        "cnf-company"
    }

    fn get_database_name() -> &'static str {
        "bod"
    }
}

#[async_trait]
impl Schema for CompanySchema {
    fn get_collection_name(&self) -> &'static str {
        Company::get_collection_name()
    }

    fn get_database_name(&self) -> &'static str {
        Company::get_database_name()
    }

    async fn set_indexes(
        &self,
        client: &Client,
    ) -> Result<Option<CreateIndexesResult>, mongodb::error::Error> {
        let collection = client
            .database(self.get_database_name())
            .collection::<ResetToken>(self.get_collection_name());
        let mut indexes: Vec<IndexModel> = vec![];
        let user_id_index = IndexModel::builder()
            .keys(doc! {"_id":1,"isDeleted":1,"isActive":1})
            .options(
                IndexOptions::builder()
                    .unique(true)
                    .name("userId".to_string())
                    .build(),
            )
            .build();
        indexes.push(user_id_index);
        let _ = IndexFunctions::delete_existing_indexes(&collection, &mut indexes).await;
        let option: Option<CreateIndexesResult> = None;
        if indexes.len() == 0 {
            return Ok(option);
        }
        Ok(Some(collection.create_indexes(indexes).await?))
    }
}
