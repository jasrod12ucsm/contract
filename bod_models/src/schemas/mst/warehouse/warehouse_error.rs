use derive_more::Display;
use ntex::{http, web};

use crate::shared::errors::BaseError;

#[derive(Debug, Display)]
pub enum WarehouseError {
    #[display(fmt = "Error Creating Warehouse")]
    CreateWarehouseError(&'static str),
    #[display(fmt = "Error Getting Warehouse")]
    GetWarehouseError(&'static str),
    #[display(fmt = "Error Deleting Warehouse")]
    DeleteWarehouseError(&'static str),
    #[display(fmt = "Error Updating Warehouse")]
    UpdateWarehouseError(&'static str),
    #[display(fmt = "Error Listing Warehouses")]
    ListWarehousesError(&'static str),
}

impl web::error::WebResponseError for WarehouseError {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        let error = match *self {
            WarehouseError::CreateWarehouseError(msg)
            | WarehouseError::GetWarehouseError(msg)
            | WarehouseError::DeleteWarehouseError(msg)
            | WarehouseError::UpdateWarehouseError(msg)
            | WarehouseError::ListWarehousesError(msg) => BaseError {
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
            WarehouseError::CreateWarehouseError(_) => http::StatusCode::BAD_REQUEST,
            WarehouseError::GetWarehouseError(_) => http::StatusCode::BAD_REQUEST,
            WarehouseError::DeleteWarehouseError(_) => http::StatusCode::BAD_REQUEST,
            WarehouseError::UpdateWarehouseError(_) => http::StatusCode::BAD_REQUEST,
            WarehouseError::ListWarehousesError(_) => http::StatusCode::BAD_REQUEST,
        }
    }
}