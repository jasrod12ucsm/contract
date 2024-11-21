use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize,Deserialize,Validate)]
pub struct CreateContract{
    pub name:String,
    pub surnames:String,
    pub address:String,
    pub role:String,
    pub birthdate:String,
    pub email:String,
    pub price:String,
    pub date_start:String,
    pub date_end:String,
    pub enterprise_name:String,
    pub enterprise_ruc:String,
    pub enterprise_represent:String,
    pub represent_dni:String,
    pub dni:String
}