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
    repositories::culqi_subscription_repository_trait::CulqiSubscriptionRepositoryTrait,
};

pub struct CulqiSubscriptionRepository {
    pub culqi_datasource: Arc<dyn CulqiDataSourceTrait>,
}

#[async_trait]
impl CulqiSubscriptionRepositoryTrait for CulqiSubscriptionRepository {
    fn new(datasource: Arc<dyn CulqiDataSourceTrait>) -> Self {
        CulqiSubscriptionRepository {
            culqi_datasource: datasource,
        }
    }

    async fn cancel_subscription(
        &self,
        id: &str,
    ) -> Result<CulqiDeleteSubscriptionResponse, CulqiError> {
        let response = self.culqi_datasource.delete_subscription(id).await?;
        Ok(response)
    }

    async fn create_subscription(
        &self,
        create_subscription: CulqiCreateSubscription,
    ) -> Result<CulqiCreateSubscriptionResponse, CulqiError> {
        let response = self.create_subscription(create_subscription).await?;
        Ok(response)
    }

    async fn get_subscription(&self, id: &str) -> Result<CulqiGetSubscriptionResponse, CulqiError> {
        let response = self.get_subscription(id).await?;
        Ok(response)
    }
}
