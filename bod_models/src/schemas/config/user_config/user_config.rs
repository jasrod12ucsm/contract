use async_trait::async_trait;
use mongodb::{
    bson::{doc, DateTime},
    options::IndexOptions,
    results::CreateIndexesResult,
    Client, IndexModel,
};
use serde::{Deserialize, Serialize};

use crate::shared::{index_functions::IndexFunctions, schema::{BaseColleccionNames, Schema}};

#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct UserConfig {
    pub names: String,
    pub surnames: String,
    #[serde(rename = "accountType")]
    pub account_type: String,
    pub email: String,
    #[serde(rename = "isAuthenticated")]
    pub is_authenticated: bool,
    pub password: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
}
pub struct PartialUserConfig {
    pub names: Option<String>,
    pub surnames: Option<String>,
    pub account_type: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub is_active: Option<String>,
    pub is_delete: Option<String>,
}

impl UserConfig {
    pub fn new_client(
        names: String,
        surnames: String,
        email: String,
        password: String,
    ) -> Self {
        Self {
            names,
            surnames,
            account_type:"P".to_string(),
            email,
            password,
            is_authenticated: false,
            is_active: true,
            is_deleted: false,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}

pub struct UserConfigSchema;

impl BaseColleccionNames for UserConfig {
    fn get_collection_name() -> &'static str {
        "cnf-user-config"
    }

    fn get_database_name() -> &'static str {
        "bod"
    }
}

#[async_trait]
impl Schema for UserConfigSchema {
    fn get_collection_name(&self) -> &'static str {
        "cnf-user-config"
    }

    fn get_database_name(&self) -> &'static str {
        "bod"
    }

    async fn set_indexes(
        &self,
        client: &Client,
    ) -> Result<Option<CreateIndexesResult>, mongodb::error::Error> {
        let collection = client
            .database(self.get_database_name())
            .collection::<UserConfig>(self.get_collection_name());
        let mut indexes: Vec<IndexModel> = vec![];
        let unique_email_index = IndexModel::builder()
            .keys(doc! {"email":1})
            .options(
                IndexOptions::builder()
                    .unique(true)
                    .name("email".to_string())
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
