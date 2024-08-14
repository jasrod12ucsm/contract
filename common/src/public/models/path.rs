use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct IdPath {
    id: String,
}

impl IdPath {
    pub fn id(&self) -> &str {
        &self.id
    }
}
