use derive_more::Display;
use ntex::{http, web};

use crate::shared::errors::BaseError;

#[derive(Debug, Display)]
pub enum ProductCategoryError {
    #[display(fmt = "Error Creating Product Category")]
    CreateProductCategoryError(&'static str),
    #[display(fmt = "Error Getting Product Category")]
    GetProductCategoryError(&'static str),
    #[display(fmt = "Error Deleting Product Category")]
    DeleteProductCategoryError(&'static str),
    #[display(fmt = "Error Updating Product Category")]
    UpdateProductCategoryError(&'static str),
    #[display(fmt = "Error Listing Product Categories")]
    ListProductCategoriesError(&'static str),
}

impl web::error::WebResponseError for ProductCategoryError {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        let error = match *self {
            ProductCategoryError::CreateProductCategoryError(msg)
            | ProductCategoryError::GetProductCategoryError(msg)
            | ProductCategoryError::DeleteProductCategoryError(msg)
            | ProductCategoryError::UpdateProductCategoryError(msg)
            | ProductCategoryError::ListProductCategoriesError(msg) => BaseError {
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
            ProductCategoryError::CreateProductCategoryError(_) => http::StatusCode::BAD_REQUEST,
            ProductCategoryError::GetProductCategoryError(_) => http::StatusCode::BAD_REQUEST,
            ProductCategoryError::DeleteProductCategoryError(_) => http::StatusCode::BAD_REQUEST,
            ProductCategoryError::UpdateProductCategoryError(_) => http::StatusCode::BAD_REQUEST,
            ProductCategoryError::ListProductCategoriesError(_) => http::StatusCode::BAD_REQUEST,
        }
    }
}