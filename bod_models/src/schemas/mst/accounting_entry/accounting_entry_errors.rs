use derive_more::Display;
use ntex::{http, web};
use serde_json::json;

#[derive(Debug, Display)]
pub enum AccountingEntryError {
    #[display(fmt = "Error Creating Accounting Entry")]
    CreateAccountingEntryError(&'static str),
    #[display(fmt = "Error Getting Accounting Entry")]
    GetAccountingEntryError(&'static str),
    #[display(fmt = "Error Getting Accounting Entries")]
    GetAccountingEntriesError(&'static str),
    #[display(fmt = "Error Updating Accounting Entry")]
    UpdateAccountingEntryError(&'static str),
    #[display(fmt = "Error Deleting Accounting Entry")]
    DeleteAccountingEntryError(&'static str),
    #[display(fmt = "Accounting Entry Not Found")]
    EntryNotFoundError(&'static str),
}

impl web::error::WebResponseError for AccountingEntryError {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        match *self {
            AccountingEntryError::CreateAccountingEntryError(msg)
            | AccountingEntryError::GetAccountingEntryError(msg)
            | AccountingEntryError::GetAccountingEntriesError(msg)
            | AccountingEntryError::UpdateAccountingEntryError(msg)
            | AccountingEntryError::DeleteAccountingEntryError(msg)
            | AccountingEntryError::EntryNotFoundError(msg) => {
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