use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct EmailTemplateAttributes {
    #[serde(rename = "templateName")]
    pub template_name: String,
    pub html: String,
    #[serde(rename = "isDelete")]
    pub is_delete: bool,
    pub updated_at: DateTime,
}
impl EmailTemplateAttributes {
    pub fn new(template_name: String, html: String, is_delete: bool, updated_at: DateTime) -> Self {
        Self {
            template_name,
            html,
            is_delete,
            updated_at,
        }
    }
}