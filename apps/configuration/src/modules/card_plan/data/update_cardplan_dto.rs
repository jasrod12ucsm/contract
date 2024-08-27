use bod_models::schemas::config::card_plan::card_plan::PriceItems;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize,Validate)]
pub struct UpdateCardPlanDto {
    pub button: Option<String>,
    pub price: Option<i32>,
    pub shape: Option<String>,
    pub items: Option<Vec<PriceItems>>,
}