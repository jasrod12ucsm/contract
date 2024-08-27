use async_trait::async_trait;
use bson::doc;
use mongodb::{results::CreateIndexesResult, Client, IndexModel};
use serde::{Deserialize, Serialize};

use crate::shared::{index_functions::IndexFunctions, schema::{BaseColleccionNames, Schema}};

#[derive(Serialize, Deserialize)]
pub struct AppVariables {
    #[serde(rename = "whatsappLink")]
    pub whatsapp_link: String,
    #[serde(rename = "instagramLink")]
    pub instagram_link: String,
    #[serde(rename = "facebookLink")]
    pub facebook_link: String,
    #[serde(rename="appName")]
    pub app_name: String,
    pub phone: String,
}
impl AppVariables {
    pub fn new(whatsapp_link: String, instagram_link: String, facebook_link: String, app_name: String,phone:String) -> Self {
        Self {
            phone,
            whatsapp_link,
            instagram_link,
            facebook_link,
            app_name,
        }
    }
}

pub struct AppVariablesSchema;

impl BaseColleccionNames for AppVariables {
    fn get_collection_name() -> &'static str {
        "app-variables"
    }

    fn get_database_name() -> &'static str {
        "app"
    }
}

#[async_trait]
impl Schema for AppVariablesSchema {
    fn get_collection_name(&self) -> &'static str {
        AppVariables::get_collection_name()
    }

    fn get_database_name(&self) -> &'static str {
        AppVariables::get_database_name()
    }

    async fn set_indexes(
        &self,
        client: &Client,
    ) -> Result<Option<CreateIndexesResult>, mongodb::error::Error> {
        let collection = client
            .database(self.get_database_name())
            .collection::<AppVariables>(self.get_collection_name());
        let mut indexes: Vec<IndexModel> = vec![];

        let _ = IndexFunctions::delete_existing_indexes(&collection, &mut indexes).await;
        let option: Option<CreateIndexesResult> = None;
        if indexes.len() == 0 {
            return Ok(option);
        }
        Ok(Some(collection.create_indexes(indexes).await?))
    }
}