use derive_more::Display;
use ntex::{http, web};
use serde_json::json;

#[derive(Debug, Display)]
pub enum CardPlanError {
    #[display(fmt = "Error Getting Card Plan")]
    GetCardPlanError(&'static str),
    #[display(fmt = "Error Getting Card Plan")]
    GetCardPlansError(&'static str),
    #[display(fmt = "Error Deleting Card Plan")]
    DeleteCardPlanError(&'static str),
    #[display(fmt = "Error Updating Card Plan")]
    UpdateCardPlanError(&'static str),
    #[display(fmt = "Card Plan Not Found")]
    CardPlanNotFoundError(&'static str),
}

impl web::error::WebResponseError for CardPlanError {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        match *self {
            CardPlanError::GetCardPlanError(msg)
            | CardPlanError::GetCardPlansError(msg)
            | CardPlanError::DeleteCardPlanError(msg)
            | CardPlanError::UpdateCardPlanError(msg)
            | CardPlanError::CardPlanNotFoundError(msg) => {
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
