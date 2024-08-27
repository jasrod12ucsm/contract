use async_trait::async_trait;
use bson::{doc, DateTime};
use mongodb::{options::IndexOptions, results::CreateIndexesResult, Client, IndexModel};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::shared::{index_functions::IndexFunctions, schema::{BaseColleccionNames, Schema}};

#[derive(Serialize, Deserialize,Validate,Debug)]
pub struct Country {
    pub code: String,
    pub fullname:String,
    pub currency:String,
    #[serde(rename = "currencySymbol")]
    pub currency_symbol:String,
    pub timezones:Vec<String>,
    pub region:String,
    #[serde(rename = "subRegion")]
    pub sub_region:String,
    pub population:i64,
    pub flag:String,
    pub langs:Vec<String>,
     #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
}
impl Country{
    pub fn new(code:String,fullname:String,currency:String,currency_symbol:String,timezones:Vec<String>,region:String,sub_region:String,population:i64,langs:Vec<String>,flag:String) -> Self{
        let now = DateTime::now();
        Self{
            flag,
            code,
            fullname,
            currency,
            currency_symbol,
            timezones,
            region,
            sub_region,
            population,
            langs,
            is_deleted:false,
            is_active:true,
            created_at:now,
            updated_at:now
        }
    }
}

pub struct CountrySchema;

impl BaseColleccionNames for Country {
    fn get_collection_name() -> &'static str {
        "loc-country"
    }

    fn get_database_name() -> &'static str {
        "bod"
    }
}

#[async_trait]
impl Schema for CountrySchema {
    fn get_collection_name(&self) -> &'static str {
        Country::get_collection_name()
    }

    fn get_database_name(&self) -> &'static str {
        Country::get_database_name()
    }

    async fn set_indexes(
        &self,
        client: &Client,
    ) -> Result<Option<CreateIndexesResult>, mongodb::error::Error> {
        let collection = client
            .database(self.get_database_name())
            .collection::<Country>(self.get_collection_name());
        let mut indexes: Vec<IndexModel> = vec![];
        let unique_email_index = IndexModel::builder()
            .keys(doc! {"code":1})
            .options(
                IndexOptions::builder()
                    .unique(true)
                    .name("code".to_string())
                    .build(),
            )
            .build();
        indexes.push(unique_email_index);
        let _ = IndexFunctions::delete_existing_indexes(&collection, &mut indexes).await;
        let option: Option<CreateIndexesResult> = None;
        if indexes.len() == 0 {
            return Ok(option);
        }
        Ok(Some(collection.create_indexes(indexes).await?))
    }
}
