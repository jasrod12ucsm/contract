use compilation_procedure::ToDatabaseQuery;
use serde::{Deserialize, Serialize};
use bod_models::shared::bson::to_document::ToDocument;
#[derive(Serialize,Deserialize,ToDatabaseQuery)]
pub struct FilterRegionByCountryCodeRegion{
    #[serde(rename="countryId")]
    pub country_id:String,
    #[serde(rename="noDeleted")]
    pub no_deleted:bool
}