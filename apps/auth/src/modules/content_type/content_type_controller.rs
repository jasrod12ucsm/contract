use bod_models::schemas::config::content_type::{
    content_type_error::ContentTypeError, models::cnf_content_type_with_id::ContentTypeWithId
};
use bson::doc;
use common::utils::ntex_private::{
    extractors::json::JsonAdvanced, repository::public_repository::{AbstractRepository, PublicRepository},
};
use futures::StreamExt;
use ntex::web::{self, types::State};

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
        .find(doc! {})
        .await
        .map_err(|_| ContentTypeError::ListContentTypesError("Cannot list content types"))?;
    let content_types: Vec<ContentTypeWithId> = content_types_cursor
        .filter_map(|result| async move { result.ok() })
        .collect()
        .await;
    Ok(JsonAdvanced(content_types))
}

