use derive_more::Display;
use ntex::{http, web};
use serde_json::json;

#[derive(Debug, Display)]
pub enum UserError {
    #[display(fmt = "Error Creating user")]
    CreateUserError(&'static str),
    #[display(fmt = "Error Getting User")]
    GetUserError(&'static str),
    #[display(fmt = "Error Deleting User")]
    GetUsersError(&'static str),
    #[display(fmt = "Error Updating User")]
    DeleteUserError(&'static str),
    #[display(fmt = "Error Updating User")]
    UpdateUserError(&'static str),
    #[display(fmt = "Password Incorrect")]
    PasswordIncorrect(&'static str),
    #[display(fmt = "Login User Error")]
    LoginUserError(&'static str),
}

impl web::error::WebResponseError for UserError {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        match *self {
            UserError::CreateUserError(msg) => web::HttpResponse::build(self.status_code())
                .set_header("content-type", "application/json; charset=utf-8")
                .json(&json!({ "error": "Error Creating user", "statusCode":404, "message":msg})),
            UserError::GetUserError(msg) => web::HttpResponse::build(self.status_code())
                .set_header(
                    ntex::http::header::CONTENT_TYPE,
                    mime::APPLICATION_JSON.to_string(),
                )
                .json(&json!({"error":"Error Getting User","statusCode":404, "message":msg})),
            UserError::DeleteUserError(msg) => web::HttpResponse::build(self.status_code())
                .set_header(
                    ntex::http::header::CONTENT_TYPE,
                    mime::APPLICATION_JSON.to_string(),
                )
                .json(&json!({"error":"Error Deleting User", "statusCode":404, "message":msg})),
            UserError::UpdateUserError(msg) => web::HttpResponse::build(self.status_code())
                .set_header(
                    ntex::http::header::CONTENT_TYPE,
                    mime::APPLICATION_JSON.to_string(),
                )
                .json(&json!({"error":"Error Deleting User", "statusCode":404, "message":msg})),
            UserError::GetUsersError(msg) => web::HttpResponse::build(self.status_code())
                .set_header(
                    ntex::http::header::CONTENT_TYPE,
                    mime::APPLICATION_JSON.to_string(),
                )
                .json(&json!({"error":"Error Getting Users User", "statusCode":404, "message":msg})),
            Self::PasswordIncorrect(msg) => web::HttpResponse::build(self.status_code())
                .set_header(
                    ntex::http::header::CONTENT_TYPE,
                    mime::APPLICATION_JSON.to_string(),
                )
                .json(&json!({"error":"Password Incorrect", "statusCode":404, "message":msg})),
            Self::LoginUserError(msg) => web::HttpResponse::build(self.status_code())
                .set_header(
                    ntex::http::header::CONTENT_TYPE,
                    mime::APPLICATION_JSON.to_string(),
                )
                .json(&json!({"error":"Login User Error", "statusCode":404, "message":msg})),
        }
    }
    fn status_code(&self) -> http::StatusCode {
        match *self {
            UserError::CreateUserError(_) => http::StatusCode::BAD_REQUEST,
            UserError::GetUserError(_) => http::StatusCode::BAD_REQUEST,
            UserError::DeleteUserError(_) => http::StatusCode::BAD_REQUEST,
            UserError::UpdateUserError(_) => http::StatusCode::BAD_REQUEST,
            UserError::GetUsersError(_) => http::StatusCode::BAD_REQUEST,
            UserError::PasswordIncorrect(_) => http::StatusCode::BAD_REQUEST,
            UserError::LoginUserError(_) => http::StatusCode::BAD_REQUEST,
        }
    }
}
