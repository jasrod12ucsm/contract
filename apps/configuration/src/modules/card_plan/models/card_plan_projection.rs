use bod_models::schemas::config::card_plan::card_plan::PriceItems;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CardPlanProjection {
    button: String,
    price: i32,
    shape: String,
    items: Vec<PriceItems>,
}
