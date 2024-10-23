use bod_models::shared::bson::to_document::ToDocument;
use compilation_procedure::ToDatabaseQuery;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, ToDatabaseQuery)]
pub struct SetLoginByTokenResetToken {
    #[serde(rename = "devices.$[d].token")]
    pub token: String,
    #[serde(rename = "devices.$[d].os")]
    pub os: String,
}
