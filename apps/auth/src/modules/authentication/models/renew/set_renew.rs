use bod_models::shared::bson::to_document::ToDocument;
use compilation_procedure::ToDatabaseQuery;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, ToDatabaseQuery)]
pub struct SetRenewResetToken {
    #[serde(rename = "devices.$[device].token")]
    pub token: String,
}
