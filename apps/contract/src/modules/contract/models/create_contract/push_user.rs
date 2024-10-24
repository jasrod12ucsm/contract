use compilation_procedure::ToDatabaseQuery;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use bod_models::{schemas::mst::user::models::contract::Contract, shared::bson::to_document::ToDocument};



#[derive(Debug,Serialize,Deserialize,Clone,ToDatabaseQuery,Builder)]
pub struct PushUpdateContract{
    contract:Contract
}