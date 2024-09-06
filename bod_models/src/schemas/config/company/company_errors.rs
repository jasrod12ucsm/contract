use derive_more::Display;
use ntex::{http, web};

use crate::shared::errors::BaseError;

#[derive(Debug, Display)]
pub enum CompanyError {
    #[display(fmt = "Error Creating Company")]
    CreateCompanyError(&'static str),
    #[display(fmt = "Error Getting Company")]
    GetCompanyError(&'static str),
    #[display(fmt = "Error Deleting Company")]
    DeleteCompanyError(&'static str),
    #[display(fmt = "Error Updating Company")]
    UpdateCompanyError(&'static str),
    #[display(fmt = "Company Not Found")]
    CompanyNotFoundError(&'static str),
}

impl web::error::WebResponseError for CompanyError {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        let error = match *self {
            CompanyError::CreateCompanyError(msg)
            | CompanyError::GetCompanyError(msg)
            | CompanyError::DeleteCompanyError(msg)
            | CompanyError::UpdateCompanyError(msg)
            | CompanyError::CompanyNotFoundError(msg) => BaseError {
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
            CompanyError::CreateCompanyError(_) => http::StatusCode::BAD_REQUEST,
            CompanyError::GetCompanyError(_) => http::StatusCode::BAD_REQUEST,
            CompanyError::DeleteCompanyError(_) => http::StatusCode::BAD_REQUEST,
            CompanyError::UpdateCompanyError(_) => http::StatusCode::BAD_REQUEST,
            CompanyError::CompanyNotFoundError(_) => http::StatusCode::NOT_FOUND,
        }
    }
}