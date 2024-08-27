use bson::{doc, oid::ObjectId, DateTime};
use mongodb::{options::IndexOptions, results::CreateIndexesResult, Client, IndexModel};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    schemas::mst::user::user::User,
    shared::{
        index_functions::IndexFunctions,
        schema::{BaseColleccionNames, Schema},
    },
};

#[derive(Serialize, Deserialize,Validate)]
pub struct AccountingAccount {
    #[serde(rename = "_id")]
    pub account_number: String,
    #[serde(rename = "accountName")]
    pub account_name: String,
    #[serde(rename = "accountType")]
    pub account_type: String,
    #[serde(rename = "accountCategory")]
    pub account_category: String,
    pub company: Option<ObjectId>,
    #[serde(rename = "parentAccount")]
    pub parent_account: Option<ObjectId>, //este atributo es para las subcuentas
    #[serde(rename = "accountBalance")]
    pub balance: f64,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
}

impl AccountingAccount {
    pub fn new(
        account_number: String,
        account_name: String,
        account_type: String,
        account_category: String,
        company: Option<ObjectId>,
        parent_account: Option<ObjectId>,
        balance: f64,
    ) -> AccountingAccount {
        AccountingAccount {
            company,
            account_number,
            account_name,
            account_type,
            account_category,
            parent_account,
            balance,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
            is_active: true,
            is_deleted: false,
        }
    }
}

pub struct AccountingAccountSchema;

impl BaseColleccionNames for AccountingAccount {
    fn get_collection_name() -> &'static str {
        "mst-accounting-account"
    }

    fn get_database_name() -> &'static str {
        "bod"
    }
}

#[async_trait::async_trait]
impl Schema for AccountingAccountSchema {
    fn get_collection_name(&self) -> &'static str {
        "mst-accounting-account"
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

        let unique_account_number = IndexModel::builder()
            .keys(doc! {"accountNumber":1})
            .options(
                IndexOptions::builder()
                    .unique(true)
                    .name("accountNumber".to_string())
                    .build(),
            )
            .build();
        let parent_id_index = IndexModel::builder()
            .keys(doc! {"parentAccount":1})
            .options(
                IndexOptions::builder()
                    .name("parentAccount".to_string())
                    .build(),
            )
            .build();
        //indices compuestos necesarios para las consultas
        let account_number_and_parent_index = IndexModel::builder()
            .keys(doc! {"accountNumber":1,"parentAccount":1})
            .options(
                IndexOptions::builder()
                    .name("accountNumberParentAccount".to_string())
                    .build(),
            )
            .build();
        let account_active_delete_index = IndexModel::builder()
            .keys(doc! {"isActive":1,"isDeleted":1})
            .options(IndexOptions::builder().name("isActive".to_string()).build())
            .build();
        //account and active y delete

        let account_number_and_active_and_delete_index = IndexModel::builder()
            .keys(doc! {"accountNumber":1,"isActive":1,"isDeleted":1})
            .options(IndexOptions::builder().name("accountNumberActiveDelete".to_string()).build())
            .build();
        indexes.push(account_number_and_active_and_delete_index);
        indexes.push(account_active_delete_index);
        indexes.push(account_number_and_parent_index);

        indexes.push(parent_id_index);

        indexes.push(unique_account_number);

        let _ = IndexFunctions::delete_existing_indexes(&collection, &mut indexes).await;
        let option: Option<CreateIndexesResult> = None;
        if indexes.len() == 0 {
            return Ok(option);
        }
        Ok(Some(collection.create_indexes(indexes).await?))
    }
}
