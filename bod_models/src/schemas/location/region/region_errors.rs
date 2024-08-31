use derive_more::Display;
use ntex::{http, web};

use crate::shared::errors::BaseError;

#[derive(Debug, Display)]
pub enum RegionError {
    #[display(fmt = "Error Creating Region")]
    CreateRegionError(String),
    #[display(fmt = "Error Getting Region")]
    GetRegionError(&'static str),
    #[display(fmt = "Error Deleting Region")]
    DeleteRegionError(String), // Add a message parameter of type String
    #[display(fmt = "Error Updating Region")]
    UpdateRegionError(&'static str),
    #[display(fmt = "Error Getting Regions")]
    GetRegionsError(&'static str),
}

impl web::error::WebResponseError for RegionError {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        let error = match self {
            RegionError::CreateRegionError(msg) | RegionError::DeleteRegionError(msg) => {
                BaseError::new(self.to_string(), msg.to_owned(), self.status_code().as_u16() as i32)
            }
            RegionError::GetRegionError(msg)
            | RegionError::UpdateRegionError(msg)
            | RegionError::GetRegionsError(msg) => BaseError {
                error: self.to_string(),
                message: msg.to_string(),
                status_code: self.status_code().as_u16() as i32,
            },
        };

        web::HttpResponse::build(self.status_code())
            .set_header("content-type", "application/json; charset=utf-8")
            .json(&error)
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
