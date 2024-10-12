use derive_more::Display;
use ntex::{http, web};

use crate::shared::errors::{BadRequestError, ErrorGenerate};

#[derive(Debug, Display)]
pub enum ContentTypeError {
    #[display(fmt = "Error Creating Content Type")]
    CreateContentTypeError(&'static str),
    #[display(fmt = "Error Getting Content Type")]
    GetContentTypeError(&'static str),
    #[display(fmt = "Error Deleting Content Type")]
    DeleteContentTypeError(&'static str),
    #[display(fmt = "Error Updating Content Type")]
    UpdateContentTypeError(&'static str),
    #[display(fmt = "Error Listing Content Types")]
    ListContentTypesError(&'static str),
}

impl web::error::WebResponseError for ContentTypeError {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        match *self {
            ContentTypeError::CreateContentTypeError(msg)
            | ContentTypeError::GetContentTypeError(msg)
            | ContentTypeError::DeleteContentTypeError(msg)
            | ContentTypeError::UpdateContentTypeError(msg)
            | ContentTypeError::ListContentTypesError(msg) => {
                return BadRequestError::render_by_status(
                    "ContentType error".to_string(),
                    msg.to_string(),
                    self.status_code(),
                );
            }
        };
    }

    fn status_code(&self) -> http::StatusCode {
        match *self {
            ContentTypeError::CreateContentTypeError(_) => http::StatusCode::BAD_REQUEST,
            ContentTypeError::GetContentTypeError(_) => http::StatusCode::BAD_REQUEST,
            ContentTypeError::DeleteContentTypeError(_) => http::StatusCode::BAD_REQUEST,
            ContentTypeError::UpdateContentTypeError(_) => http::StatusCode::BAD_REQUEST,
            ContentTypeError::ListContentTypesError(_) => http::StatusCode::BAD_REQUEST,
        }
    }
}
