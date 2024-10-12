use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Debug)]
pub struct UserId{
    pub user: String,
    pub email:String
}