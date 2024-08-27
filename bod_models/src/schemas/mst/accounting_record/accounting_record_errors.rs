use derive_more::Display;
use ntex::{http, web};
use serde_json::json;

#[derive(Debug, Display)]
pub enum AccountingRecordError {
    #[display(fmt = "Error Creating Accounting Record")]
    CreateAccountingRecordError(&'static str),
    #[display(fmt = "Error Getting Accounting Record")]
    GetAccountingRecordError(&'static str),
    #[display(fmt = "Error Getting Accounting Records")]
    GetAccountingRecordsError(&'static str),
    #[display(fmt = "Error Updating Accounting Record")]
    UpdateAccountingRecordError(&'static str),
    #[display(fmt = "Error Deleting Accounting Record")]
    DeleteAccountingRecordError(&'static str),
    #[display(fmt = "Accounting Record Not Found")]
    AccountingRecordNotFoundError(&'static str),
}

impl web::error::WebResponseError for AccountingRecordError {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        match *self {
            AccountingRecordError::CreateAccountingRecordError(msg)
            | AccountingRecordError::GetAccountingRecordError(msg)
            | AccountingRecordError::GetAccountingRecordsError(msg)
            | AccountingRecordError::UpdateAccountingRecordError(msg)
            | AccountingRecordError::DeleteAccountingRecordError(msg)
            | AccountingRecordError::AccountingRecordNotFoundError(msg) => {
                web::HttpResponse::build(self.status_code())
                    .set_header("content-type", "application/json; charset=utf-8")
                    .json(&json!({ "error": self.to_string(), "statusCode": 400, "message": msg }))
            }
        }
    }

    fn status_code(&self) -> http::StatusCode {
        http::StatusCode::BAD_REQUEST
    }
}