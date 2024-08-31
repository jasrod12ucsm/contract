use derive_more::Display;
use ntex::{http, web};

use crate::shared::errors::BaseError;

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
        let error = match *self {
            UserError::CreateUserError(msg)
            | UserError::GetUserError(msg)
            | UserError::DeleteUserError(msg)
            | UserError::UpdateUserError(msg)
            | UserError::GetUsersError(msg)
            | UserError::PasswordIncorrect(msg)
            | UserError::LoginUserError(msg) => BaseError {
                error: self.to_string(),
                message: msg.to_string(),
                status_code: self.status_code().as_u16() as i32,
            },
        };

        web::HttpResponse::build(self.status_code())
            .set_header("content-type", "application/json; charset=utf-8")
            .json(&error)
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