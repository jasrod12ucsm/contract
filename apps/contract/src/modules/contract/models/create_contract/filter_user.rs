use compilation_procedure::{ToBson, ToDatabaseQuery};
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use bod_models::shared::bson::to_document::ToDocument;
use bson::{Bson,to_bson};

#[derive(Debug, Serialize, Deserialize,ToDatabaseQuery)]
pub struct FilterUserExist {
    #[serde(rename = "$or")]
    pub or: Vec<UserFilter>,
}

#[derive(Debug, Serialize, Deserialize, Clone,ToBson)]
#[serde(untagged)]
pub enum UserFilter {
    Name { name: String },
    Surnames { surnames: String },
    Role { role: String },
    Address { address: String },
    Birthdate { birthdate: String },
    Email { email: String },
}

#[derive(Debug, Serialize, Deserialize,ToDatabaseQuery)]
pub struct FilterContractExist {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    
    #[serde(rename = "contract.init_date")]
    pub init_date: String,
    
    #[serde(rename = "contract.finish_date")]
    pub finish_date: String,
}