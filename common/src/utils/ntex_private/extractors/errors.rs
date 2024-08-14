use ntex::{http, web};
use serde::Serialize;
use serde_json::json;
//multiparte
#[derive(Debug, derive_more::Display)]
pub enum MultipartError {
    #[display(fmt = "Validation error on field:")]
    ValidationError(ValidationErrorStruct),
    #[display(fmt = "Error reading multipart data")]
    FileChargeError,
    #[display(fmt = "Error reading multipart data")]
    ValidationFieldsError(ValidationFieldsErrorStruct),
}

//json
#[derive(Debug, derive_more::Display)]
pub enum JsonError {
    #[display(fmt = "Error reading json data")]
    JsonSerializeError,
    #[display(fmt = "Error reading json data")]
    ValidationFieldsError(ValidationFieldsErrorStruct),
    #[display(fmt = "Error reading json data")]
    InternalServerError,
    #[display(fmt = "transform payload error")]
    JsonBasicTransformError,
}

#[derive(Debug, Serialize)]
pub struct ValidationFieldsErrorStruct {
    pub error: String,
    pub description: validator::ValidationErrors,
    pub status_code: u16,
}

impl ValidationFieldsErrorStruct {
    pub fn new(description: validator::ValidationErrors) -> Self {
        Self {
            error: "error en la validacion".to_string(),
            description,
            status_code: 400,
        }
    }
}
impl ValidationErrorStruct {
    pub fn new(field: Vec<String>) -> Self {
        Self {
            error: "error en la validacion".to_string(),
            field,
            status_code: 400,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ValidationErrorStruct {
    pub error: String,
    pub field: Vec<String>,
    pub status_code: u16,
}

impl web::error::WebResponseError for MultipartError {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        match *self {
            MultipartError::ValidationError(ref field) => {
                web::HttpResponse::build(self.status_code())
                    .set_header("content-type", "text/json; charset=utf-8")
                    .json(&json!(&field))
            }
            MultipartError::FileChargeError => web::HttpResponse::build(self.status_code())
                .set_header("content-type", "text/json; charset=utf-8")
                .json(&json!({"error":"file charge error","status_code":403})),
            MultipartError::ValidationFieldsError(ref field) => {
                web::HttpResponse::build(self.status_code())
                    .set_header("content-type", "text/json; charset=utf-8")
                    .json(&json!(&field))
            }
        }
    }
    fn status_code(&self) -> http::StatusCode {
        match *self {
            MultipartError::ValidationError { .. } => http::StatusCode::BAD_REQUEST,
            MultipartError::FileChargeError => http::StatusCode::FORBIDDEN,
            MultipartError::ValidationFieldsError { .. } => http::StatusCode::BAD_REQUEST,
        }
    }
}

impl web::error::WebResponseError for JsonError {
    fn status_code(&self) -> http::StatusCode {
        match *self {
            JsonError::InternalServerError => http::StatusCode::INTERNAL_SERVER_ERROR,
            JsonError::JsonSerializeError => http::StatusCode::BAD_REQUEST,
            JsonError::ValidationFieldsError(..) => http::StatusCode::BAD_REQUEST,
            JsonError::JsonBasicTransformError => http::StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self, _: &web::HttpRequest) -> http::Response {
        match *self {
            JsonError::InternalServerError => http::Response::build(self.status_code())
                .json(&json!({"error":"internal server error","status_code":403})),
            JsonError::JsonSerializeError => http::Response::build(self.status_code())
                .set_header("content-type", "text/json; charset=utf-8")
                .json(&json!({"error":"serialize error","status_code":403})),
            JsonError::ValidationFieldsError(ref field) => {
                http::Response::build(self.status_code())
                    .set_header("content-type", "text/json; charset=utf-8")
                    .json(&field)
            }
            JsonError::JsonBasicTransformError => http::Response::build(self.status_code())
                .set_header("content-type", "text/json; charset=utf-8")
                .json(&json!({"error":self.to_string(), "status_code":403})),
        }
    }
}


