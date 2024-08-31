use derive_more::Display;
use ntex::{http, web};
use serde::Serialize;

use crate::shared::errors::BaseError;


#[derive(Debug, Display,Serialize)]
pub enum CardPlanError {
    #[display(fmt = "Error Getting Card Plan")]
    GetCardPlanError(&'static str),
    #[display(fmt = "Error Getting Card Plan")]
    GetCardPlansError(&'static str),
    #[display(fmt = "Error Deleting Card Plan")]
    DeleteCardPlanError(&'static str),
    #[display(fmt = "Error Updating Card Plan")]
    UpdateCardPlanError(&'static str),
    #[display(fmt = "Card Plan Not Found")]
    CardPlanNotFoundError(&'static str),
}

impl web::error::WebResponseError for CardPlanError {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        let error = match *self {
            CardPlanError::GetCardPlanError(msg)
            | CardPlanError::GetCardPlansError(msg)
            | CardPlanError::DeleteCardPlanError(msg)
            | CardPlanError::UpdateCardPlanError(msg)
            | CardPlanError::CardPlanNotFoundError(msg) => BaseError {
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
        http::StatusCode::BAD_REQUEST
    }
}