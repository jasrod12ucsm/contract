use bod_models::shared::errors::BaseError;
use ntex::{http, web};
use serde::Serialize;




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
    #[display(fmt = "Serialize error")]
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

#[derive(Debug, Serialize)]
pub struct ValidationErrorStruct {
    pub error: String,
    pub field: Vec<String>,
    pub status_code: u16,
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

impl From<&ValidationErrorStruct> for BaseError {
    fn from(value: &ValidationErrorStruct) -> Self {
        let error = value.error.clone();
        Self {
            error: value.error.to_owned(),
            message: error,
            status_code: value.status_code as i32,
        }
    }
}

impl From<&ValidationFieldsErrorStruct> for BaseError {
    fn from(value: &ValidationFieldsErrorStruct) -> Self {
        let error = value.error.clone();
        Self {
            error: value.error.to_owned(),
            message: error,
            status_code: value.status_code as i32,
        }
    }
}

impl web::error::WebResponseError for MultipartError {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        match *self {
            MultipartError::ValidationError(ref field) => {
                let error: BaseError = field.into();
                web::HttpResponse::build(self.status_code())
                    .set_header("content-type", "text/json; charset=utf-8")
                    .json(&error)
            }
            MultipartError::FileChargeError => {
                let error = BaseError {
                    error: "file charge error".to_string(),
                    message: self.to_string(),
                    status_code: self.status_code().as_u16() as i32,
                };
                web::HttpResponse::build(self.status_code())
                    .set_header("content-type", "text/json; charset=utf-8")
                    .json(&error)
            }
            MultipartError::ValidationFieldsError(ref field) => {
                let error: BaseError = field.into();
                web::HttpResponse::build(self.status_code())
                    .set_header("content-type", "text/json; charset=utf-8")
                    .json(&error)
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

    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        match *self {
            JsonError::InternalServerError => {
                let error = BaseError {
                    error: "internal server error".to_string(),
                    message: self.to_string(),
                    status_code: self.status_code().as_u16() as i32,
                };
                web::HttpResponse::build(self.status_code())
                    .set_header("content-type", "text/json; charset=utf-8")
                    .json(&error)
            }
            JsonError::JsonSerializeError => {
                let error = BaseError {
                    error: "serialize error".to_string(),
                    message: self.to_string(),
                    status_code: self.status_code().as_u16() as i32,
                };
                web::HttpResponse::build(self.status_code())
                    .set_header("content-type", "text/json; charset=utf-8")
                    .json(&error)
            }
            JsonError::ValidationFieldsError(ref field) => {
                let error: BaseError = field.into();
                web::HttpResponse::build(self.status_code())
                    .set_header("content-type", "text/json; charset=utf-8")
                    .json(&error)
            }
            JsonError::JsonBasicTransformError => {
                let error = BaseError {
                    error: "transform payload error".to_string(),
                    message: self.to_string(),
                    status_code: self.status_code().as_u16() as i32,
                };
                web::HttpResponse::build(self.status_code())
                    .set_header("content-type", "text/json; charset=utf-8")
                    .json(&error)
            }
        }
    }
}