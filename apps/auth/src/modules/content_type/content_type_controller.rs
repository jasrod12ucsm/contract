use bod_models::schemas::config::content_type::{
    content_type::ContentTypeWithId, content_type_error::ContentTypeError,
};
use bod_models::shared::bson::to_document::ToDocument;
use bson::doc;
use common::utils::database::domain::database_query::DatabaseQueryTrait;
use common::utils::database::domain::filter_query::FilterQueryTrait;
use common::utils::database::infrastructure::database_library::DatabaseQuery;
use common::utils::ntex_private::{
    extractors::json::JsonAdvanced,
    repository::public_repository::{AbstractRepository, PublicRepository},
};
use compilation_procedure::ToDatabaseQuery;
use futures::StreamExt;
use ntex::web::{self, types::State};
use serde::{Deserialize, Serialize};

use crate::utils::infrastructure::repositories::content_type_repository::ContentTypeRepository;

#[web::get("get")]
pub async fn get_content_types(
    repo: State<PublicRepository>,
) -> Result<JsonAdvanced<Vec<ContentTypeWithId>>, ContentTypeError> {
    let content_type_repository = repo
        .get_repository::<ContentTypeRepository>()
        .await
        .map_err(|_| ContentTypeError::ListContentTypesError("Internal error"))?;
    let content_types_cursor = content_type_repository
        .find(DatabaseQuery::find())
        .await
        .map_err(|_| ContentTypeError::ListContentTypesError("Cannot list content types"))?;
    let name = NameContentTypeUpdate {
        name: "hola".to_string(),
    }
    .to_doc()
    .map_err(|_| ContentTypeError::ListContentTypesError("Cannot list content types"))?;
    println!("{:?}", name);
    let content_types: Vec<ContentTypeWithId> = content_types_cursor
        .filter_map(|result| async move { result.ok() })
        .collect()
        .await;
    Ok(JsonAdvanced(content_types))
}

#[derive(Debug, Deserialize, Serialize, ToDatabaseQuery)]
pub struct NameContentTypeUpdate {
    #[field_type(mandatory)]
    pub name: String,
}
