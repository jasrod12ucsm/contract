use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct NamePath {
    pub name: String,
}

impl NamePath {
    pub fn name(&self) -> &str {
        &self.name
    }
}
