use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct PricePath{
    pub price:String
}