use bson::oid::ObjectId;
use compilation_procedure::ToDatabaseQuery;
use bod_models::shared::bson::to_document::ToDocument;
use serde::{Deserialize, Serialize};

#[derive(ToDatabaseQuery, Serialize,Deserialize)]
pub struct FilterResendEmailResetToken{
    #[serde(rename = "userId")]
    pub user_id:ObjectId,
    #[serde(rename = "noActive")]
    pub no_active:bool,
}
impl FilterResendEmailResetToken{
    pub fn new(user_id:ObjectId) -> Self{
        FilterResendEmailResetToken{
            user_id,
            no_active:true
        }
    }
}