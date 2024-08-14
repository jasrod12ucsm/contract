use derive_more::Display;
use ntex::{http, web};
use serde_json::json;

#[derive(Debug, Display)]
pub enum CountryError {
    #[display(fmt = "Error Creating Country")]
    CreateCountryError(&'static str),
    #[display(fmt = "Error Getting Country")]
    GetCountryError(&'static str),
    #[display(fmt = "Error Deleting Country")]
    GetCOuntriesError(&'static str),
    #[display(fmt = "Error Updating Country")]
    DeleteCountryError(&'static str),
    #[display(fmt = "Error Updating Country")]
    UpdateCountryError(&'static str),
}

impl web::error::WebResponseError for CountryError {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        match self {
            CountryError::CreateCountryError(msg) => web::HttpResponse::build(self.status_code())
                .set_header("content-type", "application/json; charset=utf-8")
                .json(&json!({ "error": self.to_string(), "statusCode": 404,"message":msg})),
            CountryError::GetCountryError(msg) => web::HttpResponse::build(self.status_code())
                .set_header(
                    ntex::http::header::CONTENT_TYPE,
                    mime::APPLICATION_JSON.to_string(),
                )
                .json(&json!({"error":self.to_string(),"statusCode":404,"message":msg})),
            CountryError::DeleteCountryError(msg) => web::HttpResponse::build(self.status_code())
                .set_header(
                    ntex::http::header::CONTENT_TYPE,
                    mime::APPLICATION_JSON.to_string(),
                )
                .json(&json!({"error":self.to_string(), "statusCode":404,"message":msg})),
            CountryError::UpdateCountryError(msg) => web::HttpResponse::build(self.status_code())
                .set_header(
                    ntex::http::header::CONTENT_TYPE,
                    mime::APPLICATION_JSON.to_string(),
                )
                .json(&json!({"error":self.to_string(), "statusCode":404,"message":msg})),
            CountryError::GetCOuntriesError(msg) => web::HttpResponse::build(self.status_code())
                .set_header(
                    ntex::http::header::CONTENT_TYPE,
                    mime::APPLICATION_JSON.to_string(),
                )
                .json(&json!({"error":self.to_string(), "statusCode":404,"message":msg})),
        }
    }
    fn status_code(&self) -> http::StatusCode {
        match *self {
            CountryError::CreateCountryError(_) => http::StatusCode::BAD_REQUEST,
            CountryError::GetCountryError(_) => http::StatusCode::BAD_REQUEST,
            CountryError::DeleteCountryError(_) => http::StatusCode::BAD_REQUEST,
            CountryError::UpdateCountryError(_) => http::StatusCode::BAD_REQUEST,
            CountryError::GetCOuntriesError(_) => http::StatusCode::BAD_REQUEST,
        }
    }
}
