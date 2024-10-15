use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct ShortProductCategory{
    #[serde(rename="_id")]
    id:ObjectId,
    name:String,
    lvl:i16,
    #[serde(rename="isActive")]
    is_active:bool,
    #[serde(rename="isDeleted")]
    is_deleted:bool,
}