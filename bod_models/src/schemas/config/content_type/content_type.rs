use async_trait::async_trait;
use bson::{doc, oid::ObjectId, DateTime};
use compilation_procedure::{Database, ToInsert, WithId};
use derive_builder::Builder;
use mongodb::{options::IndexOptions, results::CreateIndexesResult, Client, IndexModel};
use serde::{Deserialize, Serialize};

use crate::shared::bson::to_document::ToDocument;
use crate::shared::{
    index_functions::IndexFunctions,
    schema::{BaseColleccionNames, Schema},
};
use bson::ser::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Builder, ToInsert, WithId, Database)]
#[database(database = "bod", collection = "mst-content-type")]
#[index(
    keys = "name:1, isDeleted:1, isActive:1",
    unique = true,
    name = "name_isDeleted_isActive"
)]
pub struct ContentType {
    pub name: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
}