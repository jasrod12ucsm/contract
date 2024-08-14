use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize,Deserialize,Validate)]
pub struct GetTokenResult{
    pub token: String,
}