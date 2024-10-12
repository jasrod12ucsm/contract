use bson::DateTime;
use serde::{Deserialize, Serialize};

use crate::schemas::config::card_plan::card_plan::{Render, RestaurantsData};

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct CardPlanWithId {
    #[serde(rename = "_id")]
    pub id: String,
    pub render: Render,
    pub price_per_restaurant: i32,
    #[serde(rename = "restaurantsData")]
    pub restaurants_data: Vec<RestaurantsData>,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
    #[serde(rename = "priceActualizedDate")]
    pub price_actualized_date: Option<DateTime>,
}

impl CardPlanWithId {
    pub fn new(
        id: String,
        render: Render,
        price_per_restaurant: i32,
        restaurants_data: Vec<RestaurantsData>,
        is_active: bool,
        updated_at: DateTime,
    ) -> Self {
        Self {
            price_actualized_date: None,
            id,
            render,
            price_per_restaurant,
            restaurants_data,
            is_active,
            updated_at,
        }
    }
}