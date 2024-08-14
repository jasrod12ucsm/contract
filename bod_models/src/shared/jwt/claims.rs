use crate::schemas::mst::user::models::user_with_id::UserWithId;
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct DefaultClaims{
    exp:usize,
    user: UserWithId
}
impl DefaultClaims{
    pub fn new(exp:usize, user:UserWithId) -> Self{
        Self{
            exp,
            user
        }
    }
    
    pub fn exp(&self) -> usize {
        self.exp
    }
    
    pub fn user(&self) -> &UserWithId {
        &self.user
    }
}