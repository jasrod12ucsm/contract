use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegionGeneratePath {
    pub country: String,
    pub secret: String,
}
