use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct RenewContract {
    pub date_start: String,
    pub date_end: String,
}
