use async_trait::async_trait;

use crate::utils::domain::{datasources::culqi_datasource_trait::CulqiDataSourceTrait, models::{create_culqi_plan::CreateCulqiPlan, culqi_create_plan_response::CulqiCreatePlanResponse, culqi_error::CulqiError}};


#[async_trait]
pub trait CulqiPlanRepositoryTrait<'a>{
    fn new(datasource: &'a dyn CulqiDataSourceTrait) -> Self; // Ya es object safe
    async fn create_plan(&self, plan:CreateCulqiPlan) -> Result<CulqiCreatePlanResponse, CulqiError>;
}