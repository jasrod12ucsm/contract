use std::sync::Arc;

use async_trait::async_trait;

use crate::utils::domain::{
    datasources::culqi_datasource_trait::CulqiDataSourceTrait,
    models::{
        culqi_create_subscription::CulqiCreateSubscription,
        culqi_create_subscription_response::CulqiCreateSubscriptionResponse,
        culqi_delete_subscription_response::CulqiDeleteSubscriptionResponse,
        culqi_error::CulqiError, culqi_get_subscription_response::CulqiGetSubscriptionResponse,
    },
};
#[async_trait]
pub trait CulqiSubscriptionRepositoryTrait {
    fn new(datasource: Arc<dyn CulqiDataSourceTrait>) -> Self;

    async fn cancel_subscription(
        &self,
        id: &str,
    ) -> Result<CulqiDeleteSubscriptionResponse, CulqiError>;

    async fn create_subscription(
        &self,
        create_subscription: CulqiCreateSubscription,
    ) -> Result<CulqiCreateSubscriptionResponse, CulqiError>;
    async fn get_subscription(&self, id: &str) -> Result<CulqiGetSubscriptionResponse, CulqiError>;
}