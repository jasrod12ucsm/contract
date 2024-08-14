
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct SecretPath{
    pub name:String,
    pub secret:String,
}

impl SecretPath {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn secret(&self) -> &str {
        &self.secret
    }
}
