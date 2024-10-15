use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::shared::geo_point::GeoPoint;


#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct ProductToStore {
    //id de la tienda
    #[serde(rename = "_id")]
    pub id: ObjectId,
    //nombre de la tienda
    pub name: String,
    pub car: bool,
    pub walk: bool,
    pub local: bool,
    pub price: f32,
    pub discount: f32,
    pub quantity: i32,
    pub frecuency: String,
    #[serde(rename = "startHour")]
    pub start_hour: String,
    #[serde(rename = "endHour")]
    pub end_hour: String,
    pub location: GeoPoint,
}
