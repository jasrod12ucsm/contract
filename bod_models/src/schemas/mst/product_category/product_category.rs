use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct ProductCategory{
    #[serde(rename="_id")]
    id:ObjectId,
    name:String,
    lvl:i16,
    #[serde(rename="subCategory")]
    sub_category: Option<Box<ProductCategory>>,
    #[serde(rename="createdBy")]
    created_by:ObjectId,
    #[serde(rename="isDefault")]
    is_default:bool,
    #[serde(rename="isActive")]
    is_active:bool,
    #[serde(rename="isDeleted")]
    is_deleted:bool,
    #[serde(rename="createdAt")]
    update_at:DateTime,
    #[serde(rename="createdAt")]
    created_at:DateTime
}