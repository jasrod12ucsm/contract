use compilation_procedure::ToDatabaseQuery;
use serde::{Deserialize, Serialize};
use bod_models::shared::bson::to_document::ToDocument;

#[derive(Debug, Serialize, Deserialize,ToDatabaseQuery)]
pub struct UpdateResendEmailResetToken{
    #[serde(rename = "authCode")]
    auth_code:i32
}
impl UpdateResendEmailResetToken{
    pub fn new(auth_code:i32) -> Self{
        UpdateResendEmailResetToken{
            auth_code
        }
    }
}