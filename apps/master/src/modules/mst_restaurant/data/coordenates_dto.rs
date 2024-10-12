use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize,Validate)]
pub struct CoordenatesDto{
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub ip: Option<String>
}