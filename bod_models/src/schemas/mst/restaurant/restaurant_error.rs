use derive_more::Display;
use ntex::{http, web};

use crate::shared::errors::BaseError;

#[derive(Debug, Display)]
pub enum RestaurantError {
    #[display(fmt = "Error Creating Restaurant")]
    CreateRestaurantError(&'static str),
    #[display(fmt = "Error Getting Restaurant")]
    GetRestaurantError(&'static str),
    #[display(fmt = "Error Deleting Restaurant")]
    DeleteRestaurantError(&'static str),
    #[display(fmt = "Error Updating Restaurant")]
    UpdateRestaurantError(&'static str),
    #[display(fmt = "Error Listing Restaurants")]
    ListRestaurantsError(&'static str),
}

impl web::error::WebResponseError for RestaurantError {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        let error = match *self {
            RestaurantError::CreateRestaurantError(msg)
            | RestaurantError::GetRestaurantError(msg)
            | RestaurantError::DeleteRestaurantError(msg)
            | RestaurantError::UpdateRestaurantError(msg)
            | RestaurantError::ListRestaurantsError(msg) => BaseError {
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
            RestaurantError::CreateRestaurantError(_) => http::StatusCode::BAD_REQUEST,
            RestaurantError::GetRestaurantError(_) => http::StatusCode::BAD_REQUEST,
            RestaurantError::DeleteRestaurantError(_) => http::StatusCode::BAD_REQUEST,
            RestaurantError::UpdateRestaurantError(_) => http::StatusCode::BAD_REQUEST,
            RestaurantError::ListRestaurantsError(_) => http::StatusCode::BAD_REQUEST,
        }
    }
}