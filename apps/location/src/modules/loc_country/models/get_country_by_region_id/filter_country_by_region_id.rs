use compilation_procedure::ToDatabaseQuery;
use serde::{Deserialize, Serialize};
use bod_models::shared::bson::to_document::ToDocument;

#[derive(Deserialize,ToDatabaseQuery,Serialize)]
pub struct FilterGetCountryByRegionId {
    pub code:String
}