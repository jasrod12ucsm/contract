use derive_more::Display;
use ntex::web;
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Serialize, Deserialize)]
#[display(fmt = "Culqi error: {}", merchant_message)]
pub struct CulqiError {
    pub object: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub merchant_message: String,
    pub user_message: String,
}

// Use default implementation for `error_response()` method
impl web::error::WebResponseError for CulqiError {
    fn status_code(&self) -> reqwest::StatusCode {
        reqwest::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        web::HttpResponse::build(self.status_code())
            .set_header("content-type", "application/json; charset=utf-8")
            .json(&self)
    }
}