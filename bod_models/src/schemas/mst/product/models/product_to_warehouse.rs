use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::shared::geo_point::GeoPoint;

#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct ProductToWarehouse {
    //id del almacen
    #[serde(rename = "_id")]
    pub id: ObjectId,
    //nombre de la tienda
    pub name: String,
    pub quantity: i32,
    pub frecuency: String,
    #[serde(rename = "startHour")]
    pub start_hour: String,
    #[serde(rename = "endHour")]
    pub end_hour: String,
    pub location: GeoPoint,
}




