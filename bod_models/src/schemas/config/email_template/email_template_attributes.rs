
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct EmailTemplateAttributes {
    #[serde(rename = "templateName")]
    pub template_name: String,
    pub html: String,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
}
impl EmailTemplateAttributes {
    pub fn new(template_name: String, html: String, is_deleted: bool) -> Self {
        Self {
            template_name,
            html,
            is_deleted,
        }
    }
}