use bson::Decimal128;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Comision {
    #[serde(rename = "tax")]
    pub tax: Decimal128,
    #[serde(rename = "rate")]
    pub rate: Decimal128,
    #[serde(rename = "fee")]
    pub fee: Decimal128, // Convertido a camelCase
    #[serde(rename = "igvPayment")]
    pub igv_payment: Decimal128, // Convertido a camelCase
    #[serde(rename = "pasarelName")]
    pub pasarel_name: String, // Convertido a camelCase
}