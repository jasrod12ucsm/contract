use derive_more::{Display, Error};
use ntex::{http, web};
use serde_json::json;

#[derive(Debug, Display, Error)]
pub enum ResetTokenError {
    #[display(fmt = "Error Creating Token")]
    CreateTokenError,
    #[display(fmt = "Error Getting Token")]
    GetTokenError,
    #[display(fmt = "Error Deleting Token")]
    GetTokensError,
    #[display(fmt = "Error Updating Token")]
    DeleteTokenError,
    #[display(fmt = "Error Updating Token")]
    UpdateTokenError,
}

impl web::error::WebResponseError for ResetTokenError {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        match *self {
            ResetTokenError::CreateTokenError => web::HttpResponse::build(self.status_code())
                .set_header("content-type", "application/json; charset=utf-8")
                .json(&json!({ "error": self.to_string(), "statudCode": 404})),
            ResetTokenError::GetTokenError => web::HttpResponse::build(self.status_code())
                .set_header(
                    ntex::http::header::CONTENT_TYPE,
                    mime::APPLICATION_JSON.to_string(),
                )
                .json(&json!({"error":self.to_string(),"statudCode":404})),
            ResetTokenError::DeleteTokenError => web::HttpResponse::build(self.status_code())
                .set_header(
                    ntex::http::header::CONTENT_TYPE,
                    mime::APPLICATION_JSON.to_string(),
                )
                .json(&json!({"error":self.to_string(), "statudCode":404})),
            ResetTokenError::UpdateTokenError => web::HttpResponse::build(self.status_code())
                .set_header(
                    ntex::http::header::CONTENT_TYPE,
                    mime::APPLICATION_JSON.to_string(),
                )
                .json(&json!({"error":self.to_string(), "statudCode":404})),
            ResetTokenError::GetTokensError => web::HttpResponse::build(self.status_code())
                .set_header(
                    ntex::http::header::CONTENT_TYPE,
                    mime::APPLICATION_JSON.to_string(),
                )
                .json(&json!({"error":self.to_string(), "statudCode":404})),
        }
    }
    fn status_code(&self) -> http::StatusCode {
        match *self {
            ResetTokenError::CreateTokenError => http::StatusCode::BAD_REQUEST,
            ResetTokenError::GetTokenError => http::StatusCode::BAD_REQUEST,
            ResetTokenError::DeleteTokenError => http::StatusCode::BAD_REQUEST,
            ResetTokenError::UpdateTokenError => http::StatusCode::BAD_REQUEST,
            ResetTokenError::GetTokensError => http::StatusCode::BAD_REQUEST,
        }
    }
}
