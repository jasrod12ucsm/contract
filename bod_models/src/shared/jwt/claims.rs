use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct DefaultClaims{
    exp:usize,
    user: ObjectId
}
impl DefaultClaims{
    pub fn new(exp:usize, user:ObjectId) -> Self{
        Self{
            exp,
            user
        }
    }
    
    pub fn exp(&self) -> usize {
        self.exp
    }
    
    pub fn user(&self) -> &ObjectId {
        &self.user
    }
}



#[derive(Debug,Serialize,Deserialize)]
pub struct RenewClaims{
    exp:usize,
}
impl RenewClaims{
    pub fn new(exp:usize) -> Self{
        Self{
            exp
        }
    }
    
    pub fn exp(&self) -> usize {
        self.exp
    }
}