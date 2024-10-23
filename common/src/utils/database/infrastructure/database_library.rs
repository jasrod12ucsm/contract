use bod_models::shared::bson::to_document::ToDocument;
use bson::Document;
use serde::Serialize;

use crate::utils::database::domain::{
    database_query::DatabaseQueryTrait, filter_query::FilterQueryTrait,
    update_definition::UpdateDefinitionTrait, update_query::UpdateQueryTrait,
};

pub struct DatabaseQuery;
impl DatabaseQueryTrait for DatabaseQuery {
    fn update() -> UpdateQuery {
        UpdateQuery::default()
    }
    
    fn find()-> FindQuery {
        FindQuery::default()
    }
}

#[derive(Clone, Default)]
pub struct UpdateQuery {
    filter: Document,
    update: Option<UpdateDefinition>,
}
#[derive(Clone, Default)]
pub struct FindQuery {
    filter: Document,
}

#[derive(Clone, Default)]
pub struct UpdateDefinition {
    set: Option<Document>,
    push: Option<Document>,
}
impl UpdateDefinitionTrait for UpdateDefinition {
    fn set<U>(&mut self, data: U) -> Self
    where
        U: ToDocument,
        U: Serialize,
    {
        let mut set = Document::new();
        set.insert("$set", data.to_doc().unwrap_or_else(|_| Document::new()));
        self.set = Some(set);
        self.to_owned()
    }

    fn construct(&self) -> Document {
        let mut docu = Document::new();
        if self.set.is_some() {
            docu.insert("$set", self.set.to_owned());
        }
        docu
    }

    fn push<U>(&mut self, data: U) -> Self
    where
        U: ToDocument,
        U: Serialize,
    {
        let mut push = Document::new();
        push.insert("$push", data.to_doc().unwrap_or_else(|_| Document::new()));
        self.push = Some(push);
        self.to_owned()
    }
}

impl UpdateQueryTrait for UpdateQuery {
    fn update(&mut self, update_definition: UpdateDefinition) -> Self {
        self.update = Some(update_definition);
        self.to_owned()
    }

    fn filter<U>(&mut self, doc: U) -> Self
    where
        U: ToDocument,
        U: Serialize,
    {
        self.filter = doc.to_doc().unwrap_or_else(|_| Document::new());
        self.to_owned()
    }

    fn create_filter_doc(&self) -> Document {
        self.filter.to_owned()
    }

    fn create_update_doc(&self) -> Document {
        if self.update.is_none() {
            Document::new()
        } else {
            self.update.as_ref().unwrap().construct()
        }
    }
}
impl FilterQueryTrait for FindQuery {
    fn filter<U>(&mut self, doc: U) -> Self
    where
        U: ToDocument,
        U: Serialize,
    {
        self.filter = doc.to_doc().unwrap_or_else(|_| Document::new());
        self.to_owned()
    }

    fn create_filter_doc(&self) -> Document {
        self.filter.to_owned()
    }
}
