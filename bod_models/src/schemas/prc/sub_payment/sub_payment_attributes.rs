use bson::{oid::ObjectId, Decimal128};
use serde::{Deserialize, Serialize};

use super::models::comission::Comision;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionPaymentAttributes {
    #[serde(rename = "igv")]
    pub igv: Decimal128,
    #[serde(rename = "priceRequested")]
    pub price_requested: Decimal128, // Convertido a camelCase
    #[serde(rename = "price")]
    pub price: Decimal128,
    #[serde(rename = "comision")]
    pub comision: Comision,
    #[serde(rename = "targetToken")]
    pub target_token: String, // Convertido a camelCase
    #[serde(rename = "companyId")]
    pub company_id: ObjectId, // Convertido a camelCase
    #[serde(rename = "cardPlanId")]
    pub card_plan_id: ObjectId, // Convertido a camelCase
    #[serde(rename = "userId")]
    pub user_id: ObjectId, // Convertido a camelCase
    #[serde(rename = "error")]
    pub error: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
}

impl SubscriptionPaymentAttributes {
    pub fn new(
        igv: Decimal128,
        price_requested: Decimal128,
        price: Decimal128,
        comision: Comision,
        target_token: String,
        company_id: ObjectId,
        card_plan_id: ObjectId,
        user_id: ObjectId,
        error: String,
    ) -> Self {
        Self {
            igv,
            price_requested,
            price,
            comision,
            target_token,
            company_id,
            card_plan_id,
            user_id,
            error,
            is_active: true,
            is_deleted: false,
        }
    }
}