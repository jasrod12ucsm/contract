use core::fmt;

#[derive(Debug)]
pub enum RequestError {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    NotFound(String),
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestError::Reqwest(err) => write!(f, "Request error: {}", err),
            RequestError::Serde(err) => write!(f, "Serialization error: {}", err),
            RequestError::NotFound(resource) => write!(f, "Resource not found: {}", resource),
        }
    }
}

impl std::error::Error for RequestError {}

impl From<reqwest::Error> for RequestError {
    fn from(err: reqwest::Error) -> RequestError {
        RequestError::Reqwest(err)
    }
}

impl From<serde_json::Error> for RequestError {
    fn from(err: serde_json::Error) -> RequestError {
        RequestError::Serde(err)
    }
}