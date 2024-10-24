use async_trait::async_trait;
use bson::{doc, oid::ObjectId, DateTime};
use compilation_procedure::{Database, ToInsert, WithId};
use derive_builder::Builder;
use mongodb::{options::IndexOptions, results::CreateIndexesResult, Client, IndexModel};
use serde::{Deserialize, Serialize};

use crate::shared::{index_functions::IndexFunctions, schema::{BaseColleccionNames, Schema}};
use crate::shared::bson::to_document::ToDocument;
use bson::ser::Error;
use super::models::{contract::Contract, identification::Identification};

#[derive(Debug, Clone, Serialize, Deserialize,Builder,ToInsert,WithId,Database)]
#[database(database = "con", collection = "mst-user")]
#[index(
    keys = "identification.identification_number:1, is_deleted:1, is_active:1",
    unique = false,
    name = "identification_isDeleted_is_active"
)]
pub struct User {
    pub identification: Identification,
    pub name: String,
    pub surnames: String,
    pub address: String,
    pub email:String,
    pub role:String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub birthdate: String,
    pub is_active: bool,
    pub is_deleted: bool,
    pub contract:Vec<Contract>,
    pub enterprise_name:String,
    pub enterprise_ruc:String,
    pub enterprise_represent:String,
    pub represent_dni:String,
}


