use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use crate::schemas::config::card_plan::card_plan::PriceItems;

#[derive(Debug, Serialize, Deserialize)]
pub struct CardPlanWithId {
    #[serde(rename="_id")]
    pub id: ObjectId,
    pub button: String,
    pub price: i32,
    pub shape: String,
    pub items: Vec<PriceItems>,
    #[serde(rename="isActive")]
    pub is_active: bool,
    #[serde(rename="updatedAt")]
    pub updated_at: DateTime,
}

impl CardPlanWithId {
    pub fn new(
        id: ObjectId,
        button: String,
        price: i32,
        shape: String,
        items: Vec<PriceItems>,
        is_active: bool,
        updated_at: DateTime,
    ) -> Self {
        Self {
            id,
            button,
            price,
            shape,
            items,
            is_active,
            updated_at,
        }
    }
}