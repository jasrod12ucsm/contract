use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize,Deserialize,Validate)]
pub struct RenewResult{

    pub success: bool,
    pub message: String,
    pub refresh_token: Option<String>,
}