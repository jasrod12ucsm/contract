use async_trait::async_trait;
use bson::{doc, oid::ObjectId, DateTime};
use mongodb::{options::IndexOptions, results::CreateIndexesResult, Client, IndexModel};
use serde::{Deserialize, Serialize};

use crate::{
    schemas::{
        config::reset_token::reset_token::ResetToken,
        location::{country::models::short_country::ShortCountry, region::region::Region},
        mst::user::models::short_user::ShortUser,
    },
    shared::{
        index_functions::IndexFunctions,
        schema::{BaseColleccionNames, Schema},
    },
};

#[derive(Serialize, Deserialize)]
pub struct Company {
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
    user: ShortUser,
    country: ShortCountry,
    region: Region,
    website: Option<String>,
    #[serde(rename="employeeCount")]
    employee_count: String,
    vision: String,
    mission: String,
    categories: Option<ObjectId>,
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

#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct Sensible{
    #[serde(rename = "creditCards")]
    credit_cards: Vec<SensibleCard>,//la que esta en la posicion 0 es la principal la que esta rindiendo actualmente
    subscription: String,
    #[serde(rename = "clientToken")]
    client_token: String,
}
#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct SensibleCard{
    token: String,
    #[serde(rename = "lastFourDigits")]
    last_four_digits: i32,
    #[serde(rename = "isUsedCard")]
    is_used_card: bool,
}


#[derive(Serialize, Deserialize,Debug,Clone)]
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
            .keys(doc! {"userId":1})
            .options(
                IndexOptions::builder()
                    .unique(true)
                    .name("userId".to_string())
                    .build(),
            )
            .build();
        //crea uno para user_config_id
        let user_config_id_index = IndexModel::builder()
            .keys(doc! {"userConfigId":1})
            .options(
                IndexOptions::builder()
                    .unique(true)
                    .name("userConfigId".to_string())
                    .build(),
            )
            .build();
        indexes.push(user_id_index);
        indexes.push(user_config_id_index);
        let _ = IndexFunctions::delete_existing_indexes(&collection, &mut indexes).await;
        let option: Option<CreateIndexesResult> = None;
        if indexes.len() == 0 {
            return Ok(option);
        }
        Ok(Some(collection.create_indexes(indexes).await?))
    }
}
