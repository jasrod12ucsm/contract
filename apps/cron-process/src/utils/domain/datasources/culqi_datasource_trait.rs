use async_trait::async_trait;

use crate::utils::domain::models::{
    create_culqi_plan::CreateCulqiPlan, culqi_create_plan_response::CulqiCreatePlanResponse, culqi_create_subscription::CulqiCreateSubscription, culqi_create_subscription_response::CulqiCreateSubscriptionResponse, culqi_delete_subscription_response::CulqiDeleteSubscriptionResponse, culqi_error::CulqiError, culqi_get_subscription_response::CulqiGetSubscriptionResponse
};

#[async_trait]
pub trait CulqiDataSourceTrait: Sync + Send {
    fn new() -> Self
    where
        Self: Sized;
    async fn create_plan(
        &self,
        plan: CreateCulqiPlan,
    ) -> Result<CulqiCreatePlanResponse, CulqiError>;
    async fn delete_subscription(
        &self,
        id: &str,
    ) -> Result<CulqiDeleteSubscriptionResponse, CulqiError>;
    async fn create_subscription(
        &self,
        create_subscription: CulqiCreateSubscription,
    ) -> Result<CulqiCreateSubscriptionResponse, CulqiError>;
    async fn get_subscription(&self,id:&str)->Result<CulqiGetSubscriptionResponse,CulqiError>;
}