use async_trait::async_trait;
use bson::{doc, oid::ObjectId, DateTime};
use mongodb::{options::IndexOptions, results::CreateIndexesResult, Client, IndexModel};
use serde::{Deserialize, Serialize};

use crate::shared::{index_functions::IndexFunctions, schema::{BaseColleccionNames, Schema}};

#[derive(Serialize, Deserialize)]
pub struct ResetToken {
    pub devices: Vec<Device>,
    #[serde(rename = "userId")]
    pub user_id: ObjectId,
    pub created: DateTime,
    #[serde(rename = "authCode")]
    pub auth_code:i32,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
}
#[derive(Serialize, Deserialize,Debug)]
pub struct Device{
    pub os:String,
    pub mac:String,
    pub token:String
}


pub struct PartialResetToken {
    pub token: String,
    pub user_id: ObjectId,
    pub created: DateTime,
    pub is_delete: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
impl ResetToken {
    pub fn new(token: String, user_id: ObjectId,auth_code:i32,mac:String, os:String) -> Self {
        Self {
            auth_code,
            devices: vec![
                Device{
                    os:mac.to_string(),
                    mac:os.to_string(),
                    token:token.to_string()
                }
            ],
            user_id,
            created: DateTime::now(),
            is_deleted: false,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}
pub struct ResetTokenSchema;
impl BaseColleccionNames for ResetToken {
    fn get_collection_name() -> &'static str {
        "cnf-reset-token"
    }

    fn get_database_name() -> &'static str {
        "bod"
    }
}
#[async_trait]
impl Schema for ResetTokenSchema {
    fn get_collection_name(&self) -> &'static str {
        ResetToken::get_collection_name()
    }

    fn get_database_name(&self) -> &'static str {
        ResetToken::get_database_name()
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
        indexes.push(user_id_index);
        let _ = IndexFunctions::delete_existing_indexes(&collection, &mut indexes).await;
        let option: Option<CreateIndexesResult> = None;
        if indexes.len() == 0 {
            return Ok(option);
        }
        Ok(Some(collection.create_indexes(indexes).await?))
    }
}
