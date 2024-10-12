use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize,Serialize,Validate)]
pub struct LoginByTokenDto{
    pub mac:String,
    pub os:String
}