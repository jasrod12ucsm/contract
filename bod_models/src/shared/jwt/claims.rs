use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultClaims {
    exp: usize,
    start: String,
    finish: String,
    id:ObjectId
}
impl DefaultClaims {
    pub fn new(exp: usize, start: String, finish: String,id:ObjectId) -> Self {
        Self { exp, start, finish,id}
    }

    pub fn exp(&self) -> usize {
        self.exp
    }

    pub fn start(&self) -> &String {
        &self.start
    }
    pub fn finish(&self) -> &String {
        &self.finish
    }
    pub fn id(&self) -> &ObjectId {
        &self.id
    }
}
