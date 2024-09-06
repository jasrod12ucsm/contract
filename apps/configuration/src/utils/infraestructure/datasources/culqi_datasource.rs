use async_trait::async_trait;
use common::helpers::env::env::ENV;
use reqwest::Client;

use crate::utils::domain::{
    datasources::culqi_datasource_trait::CulqiDataSourceTrait,
    models::{
        create_culqi_plan::CreateCulqiPlan, culqi_create_plan_response::CulqiCreatePlanResponse, culqi_delete_subscription_response::CulqiDeleteSubscriptionResponse, culqi_error::CulqiError
    },
};

pub struct CulqiDatasource {
    pub url: String,
    pub secret_key: String,
    pub client: Client,
}

#[async_trait]
impl CulqiDataSourceTrait for CulqiDatasource {
    fn new() -> Self
    where
        Self: Sized,
    {
        CulqiDatasource {
            url: "https://api.culqi.com/v2".to_string(),
            secret_key: ENV
                .get_string("CULQI_SECRET_KEY")
                .expect("Not culqi secret key was provided"),
            client: Client::new(),
        }
    }

    async fn create_plan(
        &self,
        plan: CreateCulqiPlan,
    ) -> Result<CulqiCreatePlanResponse, CulqiError> {
        let response = self
            .client
            .post(format!("{}{}", &self.url, "/plans"))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.secret_key))
            .json(&plan)
            .send()
            .await
            .map_err(|_| CulqiError {
                object: "error".to_string(),
                _type: "request_error".to_string(),
                merchant_message: "Failed to send request to Culqi API.".to_string(),
                user_message:
                    "There was an issue with the integration with Culqi. Please contact support."
                        .to_string(),
            })?;

        if response.status().is_success() {
            let created_plan = response.json::<CulqiCreatePlanResponse>().await.map_err(|_| CulqiError {
                object: "error".to_string(),
                _type: "parse_error".to_string(),
                merchant_message: "Failed to parse response from Culqi API.".to_string(),
                user_message: "There was an issue with the integration with Culqi. Please contact support.".to_string(),
            })?;
            Ok(created_plan)
        } else {
            let error_response = response.json::<CulqiError>().await.map_err(|_| CulqiError {
                object: "error".to_string(),
                _type: "parse_error".to_string(),
                merchant_message: "Failed to parse error response from Culqi API.".to_string(),
                user_message: "There was an issue with the integration with Culqi. Please contact support.".to_string(),
            })?;
            Err(error_response)
        }
    }
    async fn delete_subscription(
        &self,
        id: &str,
    ) -> Result<CulqiDeleteSubscriptionResponse, CulqiError>{
        let response= self.client
            .delete(format!("{}{}", &self.url, format!("/recurrent/subscriptions/{}", id)))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.secret_key))
            .send()
            .await
            .map_err(|_| CulqiError {
                object: "error".to_string(),
                _type: "request_error".to_string(),
                merchant_message: "Failed to send request to Culqi API.".to_string(),
                user_message:
                    "There was an issue with the integration with Culqi. Please contact support."
                        .to_string(),
            })?;
        if response.status().is_success() {
            let deleted_subscription = response.json::<CulqiDeleteSubscriptionResponse>().await.map_err(|_| CulqiError {
                object: "error".to_string(),
                _type: "parse_error".to_string(),
                merchant_message: "Failed to parse response from Culqi API.".to_string(),
                user_message: "There was an issue with the integration with Culqi. Please contact support.".to_string(),
            })?;
            Ok(deleted_subscription)
        } else {
            let error_response = response.json::<CulqiError>().await.map_err(|_| CulqiError {
                object: "error".to_string(),
                _type: "parse_error".to_string(),
                merchant_message: "Failed to parse error response from Culqi API.".to_string(),
                user_message: "There was an issue with the integration with Culqi. Please contact support.".to_string(),
            })?;
            Err(error_response)
        }
    }
}
