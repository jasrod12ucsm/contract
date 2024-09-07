use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CulqiDeleteSubscriptionResponse {
    pub id: String,
    pub delete:bool,
    pub merchant_message: String,
}