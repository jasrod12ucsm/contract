use bod_models::shared::bson::to_document::ToDocument;
use bson::Document;
use serde::Serialize;

pub trait UpdateDefinitionTrait{
    fn set<U>(&mut self, data:U) -> Self where U:ToDocument,U:Serialize;
    fn push<U>(&mut self, data:U) -> Self where U:ToDocument,U:Serialize;
    fn construct(&self) -> Document;
}