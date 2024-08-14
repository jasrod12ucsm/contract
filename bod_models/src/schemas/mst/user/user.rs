use async_trait::async_trait;
use bson::{doc, DateTime};
use mongodb::{options::IndexOptions, results::CreateIndexesResult, Client, IndexModel};
use serde::{Deserialize, Serialize};

use crate::{schemas::{config::user_config::models::short_user_config::ShortUserConfig, location::{country::models::short_country::ShortCountry, region::region::Region}}, shared::{index_functions::IndexFunctions, schema::{BaseColleccionNames, Schema}}};

use super::models::identification::Identification;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub frecuency: Option<Vec<String>>,
    pub country:ShortCountry,
    pub region:Region,
    #[serde(rename = "userConfig")]
    pub user_config: ShortUserConfig,
    pub identification: Identification,
    pub phone: String,
    pub image: Option<String>,
    #[serde(rename = "parentId")]
    pub parent_id:Option<String>,
    #[serde(rename = "childsIds")]
    pub childs_ids:Option<i32>,
    pub address: String,
    pub lvl:i32,
    #[serde(rename = "typeProvider")]
    pub type_provider: Option<String>,
    #[serde(rename = "employedBy")]
    pub employed_by: Option<i32>,
    #[serde(rename = "closeHour")]
    pub close_hour: Option<String>,
    #[serde(rename = "openHour")]
    pub open_hour: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
}

impl User {
    pub fn new_client(
        user_config: ShortUserConfig,
        identification: Identification,
        phone: String,
        address: String,
        country:ShortCountry,
        region:Region
    ) -> User {
        User {
            region,
            country,
            lvl:0,//client lvl
            frecuency: None,
            user_config,
            identification,
            phone,
            image:None,
            address,
            parent_id: None,
            childs_ids: None,
            type_provider: None,
            employed_by: None,
            close_hour: None,
            open_hour: None,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
            is_active: true,
            is_deleted: false,
        }
    }
}

pub struct UserSchema;

impl BaseColleccionNames for User {
    fn get_collection_name() -> &'static str {
        "mst-user"
    }

    fn get_database_name() -> &'static str {
        "bod"
    }
}

#[async_trait]
impl Schema for UserSchema {
    fn get_collection_name(&self) -> &'static str {
        "mst-user"
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
            .collection::<User>(self.get_collection_name());
        let mut indexes: Vec<IndexModel> = vec![];
        let unique_user_config_index = IndexModel::builder()
            .keys(doc! {"userConfig._id":1})
            .options(
                IndexOptions::builder()
                    .unique(true)
                    .name("userConfig._id".to_string())
                    .build(),
            )
            .build();

        indexes.push(unique_user_config_index);
        let unique_parent = IndexModel::builder()
            .keys(doc! {"parentId":1})
            .options(
                IndexOptions::builder()
                    .name("parentId".to_string())
                    .build(),
            )
            .build();
        indexes.push(unique_parent);
        let unique_childs = IndexModel::builder()
            .keys(doc! {"childsIds":1})
            .options(
                IndexOptions::builder()
                    .name("childsIds".to_string())
                    .build(),
            )
            .build();
        indexes.push(unique_childs);
        let employed_by_index=IndexModel::builder()
        .keys(doc! {"employedBy._id":1})
        .options(
            IndexOptions::builder()
                .name("employedBy._id".to_string())
                .build(),
        )
        .build();
        indexes.push(employed_by_index);
        let _ = IndexFunctions::delete_existing_indexes(&collection, &mut indexes).await;
        let option: Option<CreateIndexesResult> = None;
        if indexes.len() == 0 {
            return Ok(option);
        }
        Ok(Some(collection.create_indexes(indexes).await?))
    }
}
