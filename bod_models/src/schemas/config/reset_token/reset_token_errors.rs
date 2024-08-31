use derive_more::Display;
use ntex::{http, web};
use serde::Serialize;

use crate::shared::errors::BaseError;


#[derive(Debug, Display,Serialize)]
pub enum ResetTokenError {
    #[display(fmt = "Error Creating Token")]
    CreateTokenError(&'static str),
    #[display(fmt = "Error Getting Token")]
    GetTokenError(&'static str),
    #[display(fmt = "Error Deleting Token")]
    DeleteTokenError(&'static str),
    #[display(fmt = "Error Updating Token")]
    UpdateTokenError(&'static str),
    #[display(fmt = "Error Getting Tokens")]
    GetTokensError(&'static str),
}

impl web::error::WebResponseError for ResetTokenError {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        let error = match *self {
            ResetTokenError::CreateTokenError(msg)
            | ResetTokenError::GetTokenError(msg)
            | ResetTokenError::DeleteTokenError(msg)
            | ResetTokenError::UpdateTokenError(msg)
            | ResetTokenError::GetTokensError(msg) => BaseError {
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