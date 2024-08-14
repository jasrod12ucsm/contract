use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize,Deserialize,Validate)]
pub struct AuthenticatePostCode{

    pub code:i32,
}