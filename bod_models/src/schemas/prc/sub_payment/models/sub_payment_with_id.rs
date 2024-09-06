use bson::{oid::ObjectId, DateTime, Decimal128};
use serde::{Deserialize, Serialize};

use crate::schemas::prc::sub_payment::sub_payment::SubscriptionPayment;

use super::comission::Comision;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionPaymentWithId {
    #[serde(rename = "_id")]
    pub id: String,
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
    #[serde(rename = "createdAt")]
    pub created_at: DateTime, // Convertido a camelCase
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime, // Convertido a camelCase
    #[serde(rename = "error")]
    pub error: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
}

impl std::fmt::Display for SubscriptionPaymentWithId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SubscriptionPaymentWithId: {}", self.id)
    }
}

impl SubscriptionPaymentWithId {
    pub fn from_subscription_payment_and_id(payment: SubscriptionPayment, id: String) -> Self {
        Self {
            id,
            igv: payment.igv,
            price_requested: payment.price_requested,
            price: payment.price,
            comision: payment.comision,
            target_token: payment.target_token,
            company_id: payment.company_id,
            card_plan_id: payment.card_plan_id,
            created_at: payment.created_at,
            updated_at: payment.updated_at,
            error: payment.error,
            is_active: payment.is_active,
            is_deleted: payment.is_deleted,
        }
    }
}