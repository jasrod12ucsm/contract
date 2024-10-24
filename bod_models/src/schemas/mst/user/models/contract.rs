use compilation_procedure::{ToBson, ToDatabaseQuery};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use crate::shared::bson::to_document::ToDocument;
use bson::{Bson,to_bson};
#[derive(Debug,Serialize,Deserialize,Clone,Builder,ToDatabaseQuery,ToBson)]
pub struct Contract{
    pub init_date:String,
    pub finish_date:String,
    pub price: f64
}