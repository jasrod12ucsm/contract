use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
pub struct BaseError{
    pub error: String,
    pub message: String,
    pub status_code: i32,
}

impl BaseError {
    pub fn new(error: String, message: String, status_code: i32) -> Self {
        Self {
            error,
            message,
            status_code,
        }
    }
}