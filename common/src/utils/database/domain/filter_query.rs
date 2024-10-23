use bod_models::shared::bson::to_document::ToDocument;
use bson::Document;
use serde::Serialize;

pub trait FilterQueryTrait{
    fn filter<U>(&mut self, doc: U) -> Self
    where
        U: ToDocument,
        U: Serialize;

    fn create_filter_doc(&self) -> Document;
}