use async_trait::async_trait;
use bson::{doc, oid::ObjectId, DateTime};
use derive_builder::Builder;
use mongodb::{options::IndexOptions, results::CreateIndexesResult, Client, IndexModel};
use serde::{Deserialize, Serialize};

use crate::{
    schemas::{
        location::{
            country::models::short_country::{ShortCountry, ShortCountryBuilder},
            region::models::short_region::{ShortRegion, ShortRegionBuilder},
        },
        mst::user::models::atention_hour::{AtentionHour, AtentionHourBuilder},
    },
    shared::{
        geo_point::GeoPoint,
        index_functions::IndexFunctions,
        schema::{BaseColleccionNames, Schema},
    },
};

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(setter(into), build_fn(validate = "Self::validate"))]
pub struct Restaurant {
    pub location: GeoPoint,
    #[serde(rename = "openHour")]
    pub open_hour: AtentionHour,
    #[serde(rename = "closeHour")]
    pub close_hour: AtentionHour,
    #[serde(rename = "efectiveArea")]
    pub efective_area: f64,
    pub country: ShortCountry,
    pub region: ShortRegion,
    pub name: String,
    #[serde(rename = "contentTypeIds")]
    pub content_type_ids: Vec<ObjectId>,
    pub address: String,
    #[serde(rename = "numMesas")]
    pub num_mesas: i32,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "timeZone")]
    pub time_zone: String,
    #[serde(rename = "companyId")]
    pub company_id: ObjectId,
}

impl RestaurantBuilder {
    fn validate(&self) -> Result<(), String> {
        if self.location.is_none() {
            return Err("Longitude is required".into());
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
        if self.company_id.is_none() {
            return Err("Company id is required".into());
        }
        if self.content_type_ids.is_none() {
            return Err("Content type ids is required".into());
        }
        Ok(())
    }

    pub fn build_partial_update(&self) -> bson::Document {
        let mut doc = doc! {};
        if let Some(location) = &self.location {
            doc.insert("location", location);
        }
        if let Some(open_hour) = &self.open_hour {
            let open_hour = AtentionHourBuilder::default()
                .friday(open_hour.friday.clone())
                .monday(open_hour.monday.clone())
                .saturday(open_hour.saturday.clone())
                .sunday(open_hour.sunday.clone())
                .thursday(open_hour.thursday.clone())
                .tuesday(open_hour.tuesday.clone())
                .wednesday(open_hour.wednesday.clone())
                .build_partial_update();
            doc.insert("openHour", open_hour);
        }
        if let Some(close_hour) = &self.close_hour {
            let close_hour = AtentionHourBuilder::default()
                .friday(close_hour.friday.clone())
                .monday(close_hour.monday.clone())
                .saturday(close_hour.saturday.clone())
                .sunday(close_hour.sunday.clone())
                .thursday(close_hour.thursday.clone())
                .tuesday(close_hour.tuesday.clone())
                .wednesday(close_hour.wednesday.clone())
                .build_partial_update();
            doc.insert("closeHour", close_hour);
        }
        if let Some(efective_area) = &self.efective_area {
            doc.insert("efectiveArea", efective_area);
        }
        if let Some(country) = &self.country {
            let country = ShortCountryBuilder::default()
                .code(country.code.clone())
                .currency(country.currency.clone())
                .currency_symbol(country.currency_symbol.clone())
                .fullname(country.fullname.clone())
                .is_active(country.is_active)
                .build_partial_update();
            doc.insert("country", country);
        }
        if let Some(region) = &self.region {
            let region = ShortRegionBuilder::default()
                .code(region.code.clone())
                .country_id(region.country_id.clone())
                .full_name(region.full_name.clone())
                .is_active(region.is_active)
                .build_partial_update();
            doc.insert("region", region);
        }
        if let Some(name) = &self.name {
            doc.insert("name", name);
        }
        if let Some(address) = &self.address {
            doc.insert("address", address);
        }
        if let Some(num_mesas) = &self.num_mesas {
            doc.insert("numMesas", num_mesas);
        }
        if let Some(is_active) = &self.is_active {
            doc.insert("isActive", is_active);
        }
        if let Some(is_deleted) = &self.is_deleted {
            doc.insert("isDeleted", is_deleted);
        }
        if let Some(updated_at) = &self.updated_at {
            doc.insert("updatedAt", updated_at);
        }
        if let Some(created_at) = &self.created_at {
            doc.insert("createdAt", created_at);
        }
        if let Some(time_zone) = &self.time_zone {
            doc.insert("timeZone", time_zone);
        }
        if let Some(company_id) = &self.company_id {
            doc.insert("companyId", company_id);
        }
        if let Some(content_type_ids) = &self.content_type_ids {
            doc.insert("contentTypeIds", content_type_ids);
        }
        doc
    }
}

impl Restaurant {
    pub fn new(
        longitude: f64,
        latitude: f64,
        open_hour: AtentionHour,
        close_hour: AtentionHour,
        efective_area: f64,
        country: ShortCountry,
        region: ShortRegion,
        name: String,
        address: String,
        num_mesas: i32,
        time_zone: String,
        company_id: ObjectId,
        content_type_ids: Vec<ObjectId>,
    ) -> Restaurant {
        Restaurant {
            content_type_ids,
            company_id,
            time_zone,
            location: GeoPoint::new(longitude, latitude),
            open_hour,
            close_hour,
            efective_area,
            country,
            region,
            name,
            address,
            num_mesas,
            is_active: true,
            is_deleted: false,
            updated_at: DateTime::now(),
            created_at: DateTime::now(),
        }
    }
}

pub struct RestaurantSchema;

impl BaseColleccionNames for Restaurant {
    fn get_collection_name() -> &'static str {
        "mst-restaurant"
    }

    fn get_database_name() -> &'static str {
        "bod"
    }
}

#[async_trait]
impl Schema for RestaurantSchema {
    fn get_collection_name(&self) -> &'static str {
        "mst-restaurant"
    }

    fn get_database_name(&self) -> &'static str {
        "bod"
    }

    async fn set_indexes(
        &self,
        client: &Client,
    ) -> Result<Option<CreateIndexesResult>, mongodb::error::Error> {
        let collection = client
            .database(self.get_database_name())
            .collection::<Restaurant>(self.get_collection_name());
        let mut indexes: Vec<IndexModel> = vec![];

        let unique_name_index = IndexModel::builder()
            .keys(doc! {"name": 1, "isDeleted": 1, "isActive": 1})
            .options(
                IndexOptions::builder()
                    .unique(true)
                    .name("name".to_string())
                    .build(),
            )
            .build();
        indexes.push(unique_name_index);

        let country_region_index = IndexModel::builder()
            .keys(doc! {"country": 1, "region": 1, "isDeleted": 1, "isActive": 1})
            .options(
                IndexOptions::builder()
                    .name("country_region".to_string())
                    .build(),
            )
            .build();
        let geo_index = IndexModel::builder()
            .keys(doc! {"location": "2dsphere"})
            .options(
                IndexOptions::builder()
                    .name("location_geo_index".to_string())
                    .build(),
            )
            .build();
        indexes.push(geo_index);
        indexes.push(country_region_index);

        let _ = IndexFunctions::delete_existing_indexes(&collection, &mut indexes).await;
        let option: Option<CreateIndexesResult> = None;
        if indexes.is_empty() {
            return Ok(option);
        }
        Ok(Some(collection.create_indexes(indexes).await?))
    }
}
