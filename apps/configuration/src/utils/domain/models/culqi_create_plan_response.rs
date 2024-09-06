use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct CulqiCreatePlanResponse {
    pub id: String,
    pub slug:String
}