use ntex::{
    http::{Response, StatusCode},
    web::{self, WebRequest, WebResponse},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseError {
    pub error: String,
    pub message: String,
    pub status_code: i32,
}

impl BaseError {
    pub fn new(error: String, message: String, status_code: i32) -> Self {
        Self {
            error,
            message,
            status_code,
        }
    }
}

pub trait ErrorGenerate {
    fn render<Err>(err: String, msg: String) -> Response;
    fn render_by_status(err: String, msg: String, status_code: StatusCode) -> Response;
    fn render_web_response<Err>(req: WebRequest<Err>, err: String, msg: String) -> WebResponse;
}

pub struct BadRequestError;

impl ErrorGenerate for BadRequestError {
    fn render<Err>(err: String, msg: String) -> Response {
        let error = BaseError::new(err, msg, 404);
        web::HttpResponse::build(StatusCode::BAD_REQUEST)
            .set_header("content-type", "application/json; charset=utf-8")
            .json(&error)
    }

    fn render_by_status(err: String, msg: String, status_code: StatusCode) -> Response {
        let error = BaseError::new(err, msg, status_code.as_u16() as i32);
        web::HttpResponse::build(status_code)
            .set_header("content-type", "application/json; charset=utf-8")
            .json(&error)
    }

    fn render_web_response<Err>(req: WebRequest<Err>, err: String, msg: String) -> WebResponse {
        let error = BaseError::new(err, msg, 404);
        req.into_response(Response::BadRequest().json(&error))
    }
}
