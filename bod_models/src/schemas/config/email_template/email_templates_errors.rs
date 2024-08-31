use derive_more::Display;
use ntex::{http, web};
use serde::Serialize;

use crate::shared::errors::BaseError;


#[derive(Debug, Display,Serialize)]
pub enum EmailTemplateError {
    #[display(fmt = "Error Getting Email Template")]
    GetEmailTemplateError(&'static str),
    #[display(fmt = "Error Deleting Email Template")]
    DeleteEmailTemplateError(&'static str),
    #[display(fmt = "Error Updating Email Template")]
    UpdateEmailTemplateError(&'static str),
    #[display(fmt = "Template Not Found")]
    TemplateNotFoundError(&'static str),
}

impl web::error::WebResponseError for EmailTemplateError {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        let error = match *self {
            EmailTemplateError::GetEmailTemplateError(msg)
            | EmailTemplateError::DeleteEmailTemplateError(msg)
            | EmailTemplateError::UpdateEmailTemplateError(msg)
            | EmailTemplateError::TemplateNotFoundError(msg) => BaseError {
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