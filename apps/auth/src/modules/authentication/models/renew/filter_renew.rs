use bson::oid::ObjectId;
use compilation_procedure::ToDatabaseQuery;
use serde::{Deserialize, Serialize};
use bod_models::shared::bson::to_document::ToDocument;
#[derive(Debug, Serialize, Deserialize,ToDatabaseQuery)]
pub struct FilterRenewResetToken{
    user_id:ObjectId,
    no_active:bool,
}
impl FilterRenewResetToken{
    pub fn new(user_id:ObjectId) -> Self{
        FilterRenewResetToken{
            user_id,
            no_active:true
        }
    }
}