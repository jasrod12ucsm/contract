use serde::{Deserialize, Serialize};
use bson::{Bson, doc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoPoint {
    #[serde(rename = "type")]
    pub geo_type: String,
    pub coordinates: [f64; 2],
}

impl GeoPoint {
    pub fn new(longitude: f64, latitude: f64) -> Self {
        GeoPoint {
            geo_type: "Point".to_string(),
            coordinates: [longitude, latitude],
        }
    }
}

impl From<GeoPoint> for Bson {
    fn from(geo_point: GeoPoint) -> Self {
        Bson::Document(doc! {
            "type": geo_point.geo_type,
            "coordinates": geo_point.coordinates.to_vec(),
        })
    }
}