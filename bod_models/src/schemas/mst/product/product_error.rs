use derive_more::Display;
use ntex::{http, web};

use crate::shared::errors::BaseError;

#[derive(Debug, Display)]
pub enum ProductError {
    #[display(fmt = "Error Creating Product")]
    CreateProductError(&'static str),
    #[display(fmt = "Error Getting Product")]
    GetProductError(&'static str),
    #[display(fmt = "Error Deleting Product")]
    DeleteProductError(&'static str),
    #[display(fmt = "Error Updating Product")]
    UpdateProductError(&'static str),
    #[display(fmt = "Error Listing Products")]
    ListProductsError(&'static str),
}

impl web::error::WebResponseError for ProductError {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        let error = match *self {
            ProductError::CreateProductError(msg)
            | ProductError::GetProductError(msg)
            | ProductError::DeleteProductError(msg)
            | ProductError::UpdateProductError(msg)
            | ProductError::ListProductsError(msg) => BaseError {
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
            ProductError::CreateProductError(_) => http::StatusCode::BAD_REQUEST,
            ProductError::GetProductError(_) => http::StatusCode::BAD_REQUEST,
            ProductError::DeleteProductError(_) => http::StatusCode::BAD_REQUEST,
            ProductError::UpdateProductError(_) => http::StatusCode::BAD_REQUEST,
            ProductError::ListProductsError(_) => http::StatusCode::BAD_REQUEST,
        }
    }
}