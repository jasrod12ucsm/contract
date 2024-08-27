use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailTemplateWithId {
    #[serde(rename="_id")]
    pub id: ObjectId,
    #[serde(rename="templateName")]
    pub template_name: String,
    pub html: String,
    #[serde(rename="isDeleted")]
    pub is_deleted: bool,
    #[serde(rename="updatedAt")]
    pub updated_at: DateTime,
}