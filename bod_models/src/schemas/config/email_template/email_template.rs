use bson::{doc, DateTime};
use mongodb::{options::IndexOptions, results::CreateIndexesResult, Client, IndexModel};
use serde::{Deserialize, Serialize};

use crate::{schemas::config::reset_token::reset_token::ResetToken, shared::{index_functions::IndexFunctions, schema::{BaseColleccionNames, Schema}}};

#[derive(Serialize, Deserialize)]
pub struct EmailTemplate {
    #[serde(rename="templateName")]
    pub template_name: String,
    pub html: String,
    #[serde(rename="isDelete")]
    pub is_delete: bool,
    pub updated_at: DateTime,
}



impl EmailTemplate {
    pub fn new(template_name: String, html: String, is_delete: bool, updated_at: DateTime) -> Self {
        Self {
            html,
            template_name,
            is_delete,
            updated_at,
        }
    }
}

pub struct EmailTemplateSchema;
impl BaseColleccionNames for EmailTemplate {
    fn get_collection_name() -> &'static str {
        "cnf-email-templates"
    }

    fn get_database_name() -> &'static str {
        "bod"
    }
}
#[async_trait::async_trait]
impl Schema for EmailTemplateSchema {
    fn get_collection_name(&self) -> &'static str {
        EmailTemplate::get_collection_name()
    }

    fn get_database_name(&self) -> &'static str {
        EmailTemplate::get_database_name()
    }

    async fn set_indexes(
        &self,
        client: &Client,
    ) -> Result<Option<CreateIndexesResult>, mongodb::error::Error> {
        let collection = client
            .database(self.get_database_name())
            .collection::<ResetToken>(self.get_collection_name());
        let mut indexes: Vec<IndexModel> = vec![];
        //crea uno para user_config_id
        let template_names = IndexModel::builder()
            .keys(doc! {"templateName":1})
            .options(
                IndexOptions::builder()
                    .unique(true)
                    .name("templateName".to_string())
                    .build(),
            )
            .build();
        indexes.push(template_names);
        let _ = IndexFunctions::delete_existing_indexes(&collection, &mut indexes).await;
        let option: Option<CreateIndexesResult> = None;
        if indexes.len() == 0 {
            return Ok(option);
        }
        Ok(Some(collection.create_indexes(indexes).await?))
    }
}
