use derive_more::Display;
use ntex::{http, web};

use crate::shared::errors::BaseError;


#[derive(Debug, Display)]
pub enum CountryError {
    #[display(fmt = "Error Creating Country")]
    CreateCountryError(&'static str),
    #[display(fmt = "Error Getting Country")]
    GetCountryError(&'static str),
    #[display(fmt = "Error Deleting Country")]
    GetCountriesError(&'static str),
    #[display(fmt = "Error Updating Country")]
    DeleteCountryError(&'static str),
    #[display(fmt = "Error Updating Country")]
    UpdateCountryError(&'static str),
}

impl web::error::WebResponseError for CountryError {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        let error = match *self {
            CountryError::CreateCountryError(msg)
            | CountryError::GetCountryError(msg)
            | CountryError::DeleteCountryError(msg)
            | CountryError::UpdateCountryError(msg)
            | CountryError::GetCountriesError(msg) => BaseError {
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
            CountryError::CreateCountryError(_) => http::StatusCode::BAD_REQUEST,
            CountryError::GetCountryError(_) => http::StatusCode::BAD_REQUEST,
            CountryError::DeleteCountryError(_) => http::StatusCode::BAD_REQUEST,
            CountryError::UpdateCountryError(_) => http::StatusCode::BAD_REQUEST,
            CountryError::GetCountriesError(_) => http::StatusCode::BAD_REQUEST,
        }
    }
}