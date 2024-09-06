use async_trait::async_trait;

use crate::utils::domain::{
    datasources::culqi_datasource_trait::CulqiDataSourceTrait,
    models::{
        culqi_delete_subscription_response::CulqiDeleteSubscriptionResponse,
        culqi_error::CulqiError,
    },
    repositories::culqi_subscription_repository_trait::CulqiSubscriptionRepositoryTrait,
};

pub struct CulqiSubscriptionRepository<'a> {
    pub culqi_datasource: &'a dyn CulqiDataSourceTrait,
}

#[async_trait]
impl<'a> CulqiSubscriptionRepositoryTrait<'a> for CulqiSubscriptionRepository<'a> {
    fn new(datasource: &'a dyn CulqiDataSourceTrait) -> Self {
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
}