use derive_more::Display;
use ntex::{http, web};
use serde_json::json;

#[derive(Debug, Display)]
pub enum AppVariablesError {
    #[display(fmt = "Error Getting App Variable")]
    GetAppVariableError(&'static str),
}

impl web::error::WebResponseError for AppVariablesError {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        match *self {
            AppVariablesError::GetAppVariableError(msg) => {
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