use derive_more::Display;
use ntex::{http, web};
use serde_json::json;

#[derive(Debug, Display)]
pub enum UserConfigError {
    #[display(fmt = "Error Creating user")]
    CreateUserError(&'static str),
    #[display(fmt = "Error Getting User")]
    GetUserError(&'static str),
    #[display(fmt = "Error Deleting User")]
    GetUsersError(&'static str),
    #[display(fmt = "Error Deleting User")]
    DeleteUserError(&'static str),
    #[display(fmt = "Error Updating User")]
    UpdateUserError(&'static str),
    #[display(fmt = "Password Incorrect")]
    PasswordIncorrect(&'static str),
    #[display(fmt = "Login User Error")]
    LoginUserError(&'static str),
    #[display(fmt = "User Already Exists")]
    UserAlreadyExists(&'static str),
    #[display(fmt = "User Not Authenticated")]
    AuthenticateError(&'static str),
}

impl web::error::WebResponseError for UserConfigError {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        match *self {
            UserConfigError::CreateUserError(msg) |
            UserConfigError::GetUserError(msg) |
            UserConfigError::DeleteUserError(msg) |
            UserConfigError::UpdateUserError(msg) |
            UserConfigError::GetUsersError(msg) |
            UserConfigError::PasswordIncorrect(msg) |
            UserConfigError::LoginUserError(msg) |
            UserConfigError::UserAlreadyExists(msg) |
            UserConfigError::AuthenticateError(msg) => web::HttpResponse::build(self.status_code())
                .set_header("content-type", "application/json; charset=utf-8")
                .json(&json!({ "error": self.to_string(), "statudCode": 404, "message": msg })),
        }
    }

    fn status_code(&self) -> http::StatusCode {
        http::StatusCode::BAD_REQUEST
    }
}