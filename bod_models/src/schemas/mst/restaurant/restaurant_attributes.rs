use bson::doc;
use serde::{Deserialize, Serialize};

use crate::{schemas::{location::{country::models::short_country::ShortCountry, region::models::short_region::ShortRegion}, mst::user::models::atention_hour::AtentionHour}, shared::bson::to_bson::ToBson};
use derive_builder::Builder;

#[derive(Debug, Clone, Serialize, Deserialize,Builder)]
#[builder(setter(into), build_fn(validate = "Self::validate"))]
pub struct RestaurantAttributes {
    pub longitude: f64,
    pub latitude: f64,
    #[serde(rename = "openHour")]
    pub open_hour: AtentionHour,
    #[serde(rename = "closeHour")]
    pub close_hour: AtentionHour,
    #[serde(rename = "efectiveArea")]
    pub efective_area: f64,
    pub country: ShortCountry,
    pub region: ShortRegion,
    pub name: String,
    pub address: String,
    #[serde(rename = "numMesas")]
    pub num_mesas: i32,
    #[serde(rename = "timeZone")]
    pub time_zone: String,
    #[serde(rename = "isActive")]
    pub is_active:bool,
    #[serde(rename="isDeleted")]
    pub is_deleted:bool
}



impl RestaurantAttributesBuilder{
      fn validate(&self) -> Result<(), String> {
        if self.longitude.is_none() {
            return Err("Longitude is required".into());
        }
        if self.latitude.is_none() {
            return Err("Latitude is required".into());
        }
        if self.open_hour.is_none() {
            return Err("Open hour is required".into());
        }
        if self.close_hour.is_none() {
            return Err("Close hour is required".into());
        }
        if self.efective_area.is_none() {
            return Err("Efective area is required".into());
        }
        if self.country.is_none() {
            return Err("Country is required".into());
        }
        if self.region.is_none() {
            return Err("Region is required".into());
        }
        if self.name.is_none() {
            return Err("Name is required".into());
        }
        if self.address.is_none() {
            return Err("Address is required".into());
        }
        if self.num_mesas.is_none() {
            return Err("Number of tables is required".into());
        }
        if self.time_zone.is_none() {
            return Err("Time zone is required".into());
        }
        if self.is_active.is_none(){
            return  Err("no is active".into());
        }
        if self.is_deleted.is_none(){
            return  Err("no is active".into());
        }
        Ok(())
    }
}

impl ToBson for RestaurantAttributes{
    fn to_bson(&self)-> Result<bson::Document,bson::ser::Error> {
        let doc =bson::to_bson(self);
        if (&doc).is_err(){
            return Err(doc.err().unwrap().to_owned());
        }
        let document=doc! {
            "$set":doc.unwrap()
        };
        Ok(document)

    }
}
