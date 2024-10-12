use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize,Serialize,Validate)]
pub struct LoginCLientDto{
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    pub mac:String,
    pub os:String
}