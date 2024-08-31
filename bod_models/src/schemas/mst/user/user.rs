use async_trait::async_trait;
use bson::{doc, oid::ObjectId, DateTime};
use mongodb::{options::IndexOptions, results::CreateIndexesResult, Client, IndexModel};
use serde::{Deserialize, Serialize};

use crate::{schemas::location::{country::models::short_country::ShortCountry, region::models::short_region::ShortRegion}, shared::{index_functions::IndexFunctions, schema::{BaseColleccionNames, Schema}}};

use super::models::{atention_hour::AtentionHour, identification::Identification};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub frecuency: Option<Vec<String>>,
    pub country:ShortCountry,
    pub region:ShortRegion,
    #[serde(rename = "userConfigId")]
    pub user_config: ObjectId,
    pub identification: Identification,
    pub phone: String,
    pub image: Option<String>,
    #[serde(rename = "parentId")]
    pub parent_id:Option<String>,
    #[serde(rename = "childsIds")]
    pub childs_ids:Option<Vec<ObjectId>>,
    pub address: String,
    pub lvl:i32,
    #[serde(rename = "typeProvider")]
    pub type_provider: String,
    #[serde(rename = "employedBy")]
    pub employed_by: Option<ObjectId>,
    #[serde(rename = "closeHour")]
    pub close_hour: AtentionHour,
    #[serde(rename = "openHour")]
    pub open_hour: AtentionHour,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
    pub birthdate: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
}

impl User {
    pub fn new_client(
        user_config: ObjectId,
        identification: Identification,
        phone: String,
        address: String,
        country:ShortCountry,
        region:ShortRegion,
        birthdate: String,
        type_provider: String,
    ) -> User {
        User {
            birthdate,
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
            type_provider: type_provider,
            employed_by: None,
            close_hour: AtentionHour::create_empty(),
            open_hour: AtentionHour::create_empty(),
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
            .keys(doc! {"userConfigId":1,"isDeleted":1,"isActive":1})
            .options(
                IndexOptions::builder()
                    .unique(true)
                    .name("userConfigId".to_string())
                    .build(),
            )
            .build();

        indexes.push(unique_user_config_index);
        let unique_parent = IndexModel::builder()
            .keys(doc! {"parentId":1,"isDeleted":1,"isActive":1})
            .options(
                IndexOptions::builder()
                    .name("parentId".to_string())
                    .build(),
            )
            .build();
        indexes.push(unique_parent);
        let unique_childs = IndexModel::builder()
            .keys(doc! {"childsIds":1,"isDeleted":1,"isActive":1})
            .options(
                IndexOptions::builder()
                    .name("childsIds".to_string())
                    .build(),
            )
            .build();
        indexes.push(unique_childs);
        let employed_by_index=IndexModel::builder()
        .keys(doc! {"employedBy":1,"isDeleted":1,"isActive":1})
        .options(
            IndexOptions::builder()
                .name("employedBy".to_string())
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
