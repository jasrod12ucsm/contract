use derive_more::Display;
use ntex::{http, web};
use serde_json::json;

#[derive(Debug, Display)]
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
        match *self {
            ResetTokenError::CreateTokenError(msg)
            | ResetTokenError::GetTokenError(msg)
            | ResetTokenError::DeleteTokenError(msg)
            | ResetTokenError::UpdateTokenError(msg)
            | ResetTokenError::GetTokensError(msg) => {
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