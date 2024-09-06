use derive_more::Display;
use ntex::{http, web};

use crate::shared::errors::BaseError;

#[derive(Debug, Display)]
pub enum SubscriptionPaymentError {
    #[display(fmt = "Failed to Create Subscription Payment")]
    CreateSubscriptionPaymentFailed(&'static str),
    #[display(fmt = "Failed to Retrieve Subscription Payment")]
    RetrieveSubscriptionPaymentFailed(&'static str),
    #[display(fmt = "Failed to Delete Subscription Payment")]
    DeleteSubscriptionPaymentFailed(&'static str),
    #[display(fmt = "Failed to Update Subscription Payment")]
    UpdateSubscriptionPaymentFailed(&'static str),
    #[display(fmt = "Failed to Process Subscription Payment")]
    ProcessSubscriptionPaymentFailed(&'static str),
}

impl web::error::WebResponseError for SubscriptionPaymentError {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        let error = match *self {
            SubscriptionPaymentError::CreateSubscriptionPaymentFailed(msg)
            | SubscriptionPaymentError::RetrieveSubscriptionPaymentFailed(msg)
            | SubscriptionPaymentError::DeleteSubscriptionPaymentFailed(msg)
            | SubscriptionPaymentError::UpdateSubscriptionPaymentFailed(msg)
            | SubscriptionPaymentError::ProcessSubscriptionPaymentFailed(msg) => BaseError {
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
            SubscriptionPaymentError::CreateSubscriptionPaymentFailed(_) => http::StatusCode::BAD_REQUEST,
            SubscriptionPaymentError::RetrieveSubscriptionPaymentFailed(_) => http::StatusCode::BAD_REQUEST,
            SubscriptionPaymentError::DeleteSubscriptionPaymentFailed(_) => http::StatusCode::BAD_REQUEST,
            SubscriptionPaymentError::UpdateSubscriptionPaymentFailed(_) => http::StatusCode::BAD_REQUEST,
            SubscriptionPaymentError::ProcessSubscriptionPaymentFailed(_) => http::StatusCode::BAD_REQUEST,
        }
    }
}