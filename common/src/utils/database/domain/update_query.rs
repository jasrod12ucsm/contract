use bod_models::shared::bson::to_document::ToDocument;
use bson::Document;
use serde::Serialize;

use crate::utils::database::infrastructure::database_library::UpdateDefinition;


pub trait UpdateQueryTrait {
    fn update(&mut self,update_definition:UpdateDefinition) -> Self;
    fn filter<U>(&mut self, doc:U) -> Self where U:ToDocument,U:Serialize;
    fn create_filter_doc(&self) -> Document;
    fn create_update_doc(&self) -> Document;
}