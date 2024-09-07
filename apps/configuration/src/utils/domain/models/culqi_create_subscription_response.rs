use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CulqiCreateSubscriptionResponse{
    id:String,
    customer_id:String,
    plan_id:String,
    status:i32,
    created_at:i64,
}