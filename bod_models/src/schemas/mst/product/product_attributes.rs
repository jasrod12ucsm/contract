use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use crate::schemas::mst::product_category::models::short_product_category::ShortProductCategory;

use super::models::{product_to_store::ProductToStore, product_to_warehouse::ProductToWarehouse};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductAttributes{
#[serde(rename = "_id")]
    pub id: String, //example
    pub name: String,
    #[serde(rename = "codeId")]
    pub code_id: String,
    #[serde(rename = "unitMeasure")]
    pub unit_measure: String,
    #[serde(rename = "parentId")]
    pub parent_id: ObjectId,
    pub description: String,
    #[serde(rename = "descriptionLong")]
    pub descrription_long: String,
    pub discount: i32,
    #[serde(rename = "storeId")]
    pub store_id: ObjectId,
    #[serde(rename = "categoryId")]
    pub categories: Vec<ShortProductCategory>,
    pub image: String,
    #[serde(rename = "smallImage")]
    pub tax: f32,
    pub small_image: String,
    #[serde(rename = "productImages")]
    pub product_images: Vec<String>,
    pub barcode: String,
    #[serde(rename = "hasVariants")]
    pub has_variants: bool,
    pub company_id: ObjectId,
    #[serde(rename = "materialVariants")]
    pub material_variants: Vec<ObjectId>,
    pub position: i32,
    #[serde(rename = "productToStore")]
    pub product_to_store: Vec<ProductToStore>,
    #[serde(rename = "productToWarehouse")]
    pub product_to_warehouse: Vec<ProductToWarehouse>,
    pub active_app: bool,
    #[serde(rename = "createdBy")]
    pub created_by: ObjectId,
    pub active_pos: bool,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
}