use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize,Validate)]
pub struct CoordenatesDto{
    pub longitude: f64,
    pub latitude: f64
}