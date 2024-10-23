use bson::{ser::Error, Document};
use serde::Serialize;

pub trait ToDocument {
    fn to_doc(&self) -> Result<Document, Error>
    where
        Self: Serialize,
    {
        let doc = bson::to_bson(self)?;
        let document = doc.as_document().ok_or_else(||{
            Error::InvalidCString("Error converting to document".to_string())
        })?;
        Ok(document.to_owned())
    }
}
