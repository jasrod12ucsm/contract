use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize,Serialize,Debug,Validate)]
pub struct RenewTokenDto {
    pub os: String,
}