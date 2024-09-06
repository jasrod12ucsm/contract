use async_trait::async_trait;

use crate::utils::domain::{datasources::culqi_datasource_trait::CulqiDataSourceTrait, models::{
    culqi_delete_subscription_response::CulqiDeleteSubscriptionResponse, culqi_error::CulqiError,
}};
#[async_trait]
pub trait CulqiSubscriptionRepositoryTrait<'a> {
    fn new(datasource: &'a dyn CulqiDataSourceTrait) -> Self;

    async fn cancel_subscription(
        &self,
        id: &str,
    ) -> Result<CulqiDeleteSubscriptionResponse, CulqiError>;
}
