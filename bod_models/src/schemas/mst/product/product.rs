use bson::{doc, oid::ObjectId, Bson, DateTime, Document};
use compilation_procedure::{Database, ToInsert, WithId};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::models::{product_to_store::ProductToStore, product_to_warehouse::ProductToWarehouse};
use crate::schemas::mst::product_category::models::short_product_category::ShortProductCategory;
use crate::shared::bson::to_document::ToDocument;
use crate::shared::index_functions::IndexFunctions;
use crate::shared::schema::{BaseColleccionNames, Schema};
use async_trait::async_trait;
use bson::ser::Error;
use mongodb::results::CreateIndexesResult;
use mongodb::{Client, IndexModel,options::IndexOptions};

#[derive(Serialize, Deserialize, Debug, Clone, Builder, ToInsert, WithId, Database)]
#[database(database = "bod", collection = "mst-product")]
#[index(
    keys = "name:1, isDeleted:1, isActive:1",
    unique = false,
    name = "name_isDeleted_isActive"
)]
pub struct Product {
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
    pub description_long: String,
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
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
}
