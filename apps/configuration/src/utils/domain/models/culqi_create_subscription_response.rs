use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CulqiCreateSubscriptionResponse {
    pub id: String,
    pub customer_id: String,
    pub plan_id: String,
    pub status: i32,
    pub created_at: i64,
}
