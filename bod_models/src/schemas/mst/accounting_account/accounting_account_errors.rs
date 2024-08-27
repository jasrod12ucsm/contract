use derive_more::Display;
use ntex::{http, web};
use serde_json::json;

#[derive(Debug, Display)]
pub enum AccountingAccountError {
    #[display(fmt = "Error Getting Accounting Account")]
    GetAccountingAccountError(&'static str),
    #[display(fmt = "Error Getting Accounting Accounts")]
    GetAccountingAccountsError(&'static str),
    #[display(fmt = "Error Deleting Accounting Account")]
    DeleteAccountingAccountError(&'static str),
    #[display(fmt = "Error Updating Accounting Account")]
    UpdateAccountingAccountError(&'static str),
    #[display(fmt = "Error Creating Accounting Account")]
    CreateAccountingAccountError(&'static str),
    #[display(fmt = "Accounting Account Not Found")]
    AccountNotFoundError(&'static str),
}

impl web::error::WebResponseError for AccountingAccountError {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        match *self {
            AccountingAccountError::GetAccountingAccountError(msg)
            | AccountingAccountError::GetAccountingAccountsError(msg)
            | AccountingAccountError::DeleteAccountingAccountError(msg)
            | AccountingAccountError::UpdateAccountingAccountError(msg)
            | AccountingAccountError::CreateAccountingAccountError(msg)
            | AccountingAccountError::AccountNotFoundError(msg) => {
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