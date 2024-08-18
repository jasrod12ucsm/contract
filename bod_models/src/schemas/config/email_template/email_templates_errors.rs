use derive_more::Display;
use ntex::{http, web};
use serde_json::json;

#[derive(Debug, Display)]
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
        match *self {
            EmailTemplateError::GetEmailTemplateError(msg)
            | EmailTemplateError::DeleteEmailTemplateError(msg)
            | EmailTemplateError::UpdateEmailTemplateError(msg)
            | EmailTemplateError::TemplateNotFoundError(msg) => {
                web::HttpResponse::build(self.status_code())
                    .set_header("content-type", "application/json; charset=utf-8")
                    .json(&json!({ "error": self.to_string(), "statusCode": 404, "message": msg }))
            }
        }
    }

    fn status_code(&self) -> http::StatusCode {
        http::StatusCode::BAD_REQUEST
    }
}
