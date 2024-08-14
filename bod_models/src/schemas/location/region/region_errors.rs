use derive_more::Display;
use ntex::{http, web};
use serde_json::json;

#[derive(Debug, Display)]
pub enum RegionError {
    #[display(fmt = "Error Creating Region")]
    CreateRegionError(String),
    #[display(fmt = "Error Getting Region")]
    GetRegionError(&'static str),
    #[display(fmt = "Error Deleting Region")]
    DeleteRegionError(&'static str),
    #[display(fmt = "Error Updating Region")]
    UpdateRegionError(&'static str),
    #[display(fmt = "Error Getting Regions")]
    GetRegionsError(&'static str),
}

impl web::error::WebResponseError for RegionError {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        match self {
            RegionError::CreateRegionError(msg) => web::HttpResponse::build(self.status_code())
                .set_header("content-type", "application/json; charset=utf-8")
                .json(&json!({ "error": self.to_string(), "statusCode": 400, "message": msg })),
            RegionError::GetRegionError(msg) => web::HttpResponse::build(self.status_code())
                .set_header(
                    ntex::http::header::CONTENT_TYPE,
                    mime::APPLICATION_JSON.to_string(),
                )
                .json(&json!({"error": self.to_string(), "statusCode": 400, "message": msg})),
            RegionError::DeleteRegionError(msg) => web::HttpResponse::build(self.status_code())
                .set_header(
                    ntex::http::header::CONTENT_TYPE,
                    mime::APPLICATION_JSON.to_string(),
                )
                .json(&json!({"error": self.to_string(), "statusCode": 400, "message": msg})),
            RegionError::UpdateRegionError(msg) => web::HttpResponse::build(self.status_code())
                .set_header(
                    ntex::http::header::CONTENT_TYPE,
                    mime::APPLICATION_JSON.to_string(),
                )
                .json(&json!({"error": self.to_string(), "statusCode": 400, "message": msg})),
            RegionError::GetRegionsError(msg) => web::HttpResponse::build(self.status_code())
                .set_header(
                    ntex::http::header::CONTENT_TYPE,
                    mime::APPLICATION_JSON.to_string(),
                )
                .json(&json!({"error": self.to_string(), "statusCode": 400, "message": msg})),
        }
    }

    fn status_code(&self) -> http::StatusCode {
        match *self {
            RegionError::CreateRegionError(_) => http::StatusCode::BAD_REQUEST,
            RegionError::GetRegionError(_) => http::StatusCode::BAD_REQUEST,
            RegionError::DeleteRegionError(_) => http::StatusCode::BAD_REQUEST,
            RegionError::UpdateRegionError(_) => http::StatusCode::BAD_REQUEST,
            RegionError::GetRegionsError(_) => http::StatusCode::BAD_REQUEST,
        }
    }
}